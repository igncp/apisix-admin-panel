import dynamic from "next/dynamic";

const LogIn = dynamic(() => import("../components/LogIn"), {
  ssr: false,
});

export default function MainPage() {
  return <LogIn />;
}
