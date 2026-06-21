import type { FieldType, WorkflowStatus } from "../types/api";
import type { MessageKey } from "./messages";

export function workflowStatusKey(status: WorkflowStatus): MessageKey {
  switch (status) {
    case "draft":
      return "common.draft";
    case "pending_review":
      return "common.pendingReview";
    case "published":
      return "common.published";
    case "archived":
      return "common.archived";
  }
}

export function workflowActionKey(status: WorkflowStatus): MessageKey {
  switch (status) {
    case "draft":
      return "common.submit";
    case "pending_review":
      return "common.publish";
    case "published":
      return "common.archive";
    case "archived":
      return "common.restore";
  }
}

export function fieldTypeKey(type: FieldType): MessageKey {
  switch (type) {
    case "text":
      return "fieldType.text";
    case "longtext":
      return "fieldType.longtext";
    case "richtext":
      return "fieldType.richtext";
    case "number":
      return "fieldType.number";
    case "boolean":
      return "fieldType.boolean";
    case "datetime":
      return "fieldType.datetime";
    case "media":
      return "fieldType.media";
  }
}