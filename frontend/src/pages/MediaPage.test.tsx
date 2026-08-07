import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import type { MediaResponse } from "../types/api";
import { ApiError } from "../services/api";
import { MediaPage } from "./MediaPage";

const apiMocks = vi.hoisted(() => ({
  list: vi.fn(),
  upload: vi.fn(),
  update: vi.fn(),
  delete: vi.fn(),
  download: vi.fn(),
}));

vi.mock("../services/api", () => ({
  ApiError: class ApiError extends Error {
    constructor(_status: number, message: string) {
      super(message);
    }
  },
  api: {
    baseUrl: "https://api.example.invalid",
    media: apiMocks,
  },
}));

function media(overrides: Partial<MediaResponse>): MediaResponse {
  return {
    id: "media-id",
    filename: "asset.webp",
    url: "/uploads/public/media/org/media-id/original.webp",
    mime_type: "image/webp",
    size: 128,
    alt_text: null,
    caption: null,
    uploader_id: null,
    visibility: "public",
    verification_status: "verified",
    lifecycle_status: "active",
    created_at: "2026-07-29T00:00:00Z",
    updated_at: "2026-07-29T00:00:00Z",
    ...overrides,
  };
}

describe("MediaPage file security UX", () => {
  afterEach(cleanup);

  beforeEach(() => {
    vi.clearAllMocks();
    apiMocks.list.mockResolvedValue({
      data: [
        media({ id: "image", filename: "safe.webp" }),
        media({
          id: "document",
          filename: "report.pdf",
          url: "/api/media/document/download",
          mime_type: "application/pdf",
          visibility: "restricted",
        }),
        media({
          id: "legacy-svg",
          filename: "<img src=x onerror=alert(1)>.svg",
          url: "/api/media/legacy-svg/download",
          mime_type: "image/svg+xml",
          visibility: "restricted",
          verification_status: "legacy_unverified",
        }),
      ],
    });
    apiMocks.download.mockResolvedValue(new Blob(["document"], { type: "application/pdf" }));
    Object.defineProperty(URL, "createObjectURL", {
      configurable: true,
      value: vi.fn(() => "blob:phase-seven"),
    });
    Object.defineProperty(URL, "revokeObjectURL", {
      configurable: true,
      value: vi.fn(),
    });
  });

  it("only inlines verified public raster images and makes restricted files downloads", async () => {
    render(<MediaPage />);

    expect(await screen.findByText("safe.webp")).toBeInTheDocument();
    expect(screen.getAllByRole("img")).toHaveLength(1);
    expect(screen.getByRole("img")).toHaveAttribute(
      "src",
      "https://api.example.invalid/uploads/public/media/org/media-id/original.webp",
    );
    expect(screen.getAllByRole("button", { name: "Download attachment" })).toHaveLength(2);
    expect(screen.getAllByRole("button", { name: "Copy media URL" })).toHaveLength(1);
    expect(screen.getByText("<img src=x onerror=alert(1)>.svg")).toBeInTheDocument();
  });

  it("treats the browser accept list as advisory and blocks an oversized selection before upload", async () => {
    render(<MediaPage />);
    await screen.findByText("safe.webp");
    const input = screen.getByLabelText("File") as HTMLInputElement;
    expect(input.accept).toBe(
      "image/jpeg,image/png,image/webp,application/pdf,text/plain",
    );

    const oversized = new File(["x"], "large.pdf", { type: "application/pdf" });
    Object.defineProperty(oversized, "size", { value: 25 * 1024 * 1024 + 1 });
    fireEvent.change(input, { target: { files: [oversized] } });

    expect(
      screen.getByText(
        "Files must be 25 MB or smaller. The backend still validates actual content and limits.",
      ),
    ).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Upload" })).toBeDisabled();
    expect(apiMocks.upload).not.toHaveBeenCalled();
  });

  it("downloads through the authenticated API as a short-lived blob URL", async () => {
    const click = vi
      .spyOn(HTMLAnchorElement.prototype, "click")
      .mockImplementation(() => undefined);
    render(<MediaPage />);

    const downloads = await screen.findAllByRole("button", { name: "Download attachment" });
    fireEvent.click(downloads[0]);

    await waitFor(() => expect(apiMocks.download).toHaveBeenCalledWith("document", expect.any(AbortSignal)));
    expect(URL.createObjectURL).toHaveBeenCalledTimes(1);
    expect(URL.revokeObjectURL).toHaveBeenCalledWith("blob:phase-seven");
    expect(click).toHaveBeenCalledTimes(1);
    click.mockRestore();
  });

  it("cancels an in-flight upload and does not submit it twice", async () => {
    apiMocks.upload.mockImplementation(
      (_file: File, _metadata: unknown, signal: AbortSignal) =>
        new Promise((_resolve, reject) => {
          signal.addEventListener("abort", () =>
            reject(new DOMException("aborted", "AbortError")),
          );
        }),
    );
    render(<MediaPage />);
    await screen.findByText("safe.webp");
    const input = screen.getByLabelText("File");
    fireEvent.change(input, {
      target: { files: [new File(["hello"], "note.txt", { type: "text/plain" })] },
    });
    fireEvent.click(screen.getByRole("button", { name: "Upload" }));

    const cancel = await screen.findByRole("button", { name: "Cancel upload" });
    expect(screen.getByRole("button", { name: "Working..." })).toBeDisabled();
    fireEvent.click(cancel);

    await waitFor(() => expect(apiMocks.upload).toHaveBeenCalledTimes(1));
    const signal = apiMocks.upload.mock.calls[0][2] as AbortSignal;
    expect(signal.aborted).toBe(true);
  });

  it("renders a server policy rejection as text without executing markup", async () => {
    apiMocks.upload.mockRejectedValue(
      new ApiError(422, "<img src=x onerror=alert('upload')> rejected by server policy"),
    );
    render(<MediaPage />);
    await screen.findByText("safe.webp");
    const input = screen.getByLabelText("File");
    fireEvent.change(input, {
      target: { files: [new File(["<html>"], "photo.jpg", { type: "image/jpeg" })] },
    });
    fireEvent.click(screen.getByRole("button", { name: "Upload" }));

    expect(
      await screen.findByText(
        "<img src=x onerror=alert('upload')> rejected by server policy",
      ),
    ).toBeInTheDocument();
    expect(screen.getAllByRole("img")).toHaveLength(1);
  });
});
