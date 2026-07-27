import { PropsWithChildren, useEffect } from "react";

import { useAppStore } from "../stores/useAppStore";

export function SessionBootstrap({ children }: PropsWithChildren) {
  const bootstrapSession = useAppStore((state) => state.bootstrapSession);

  useEffect(() => {
    void bootstrapSession();
  }, [bootstrapSession]);

  return children;
}
