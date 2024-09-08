import type { PropsWithChildren } from "react";

import TopBar from "./ui/TopBar";

export default function Layout({ children }: PropsWithChildren) {
  return (
    <>
      <TopBar />
      <div className="m-auto max-w-[1024px] pb-[50px] pt-[100px]">
        {children}
      </div>
    </>
  );
}
