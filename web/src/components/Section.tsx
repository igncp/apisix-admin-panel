import type { PropsWithChildren, ReactNode } from "react";
import { useState } from "react";

import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import Accordion from "@mui/material/Accordion";
import AccordionDetails from "@mui/material/AccordionDetails";
import AccordionSummary from "@mui/material/AccordionSummary";

import Button from "./ui/Button";
import { CloseIcon, IconButton } from "./ui/icons/Icons";

const sectionClasses = "border-[1px] border-[#333] px-[12px]";

export const Section = ({
  children,
  title,
}: PropsWithChildren<{ title: ReactNode }>) => (
  <div className={sectionClasses}>
    <Accordion>
      <AccordionSummary
        aria-controls="panel2-content"
        expandIcon={<ExpandMoreIcon />}
        id="panel2-header"
      >
        {title}
      </AccordionSummary>
      <AccordionDetails>{children}</AccordionDetails>
    </Accordion>
  </div>
);

const formClasses =
  "flex flex-col gap-[12px] items-start w-full py-[12px] p-[12px]";

type FormProps = PropsWithChildren<{
  onSubmit: (o: { onComplete: () => void }) => void;
}>;

export const Form = ({ children, onSubmit }: FormProps) => {
  const [isOpened, setIsOpened] = useState(false);

  if (!isOpened) {
    return (
      <Button
        className="border-[1px] border-[#333] px-[12px]"
        onClick={() => {
          setIsOpened(true);
        }}
      >
        New
      </Button>
    );
  }

  return (
    <form
      className={formClasses}
      onSubmit={(e) => {
        e.preventDefault();

        onSubmit({
          onComplete: () => {
            setIsOpened(false);
          },
        });
      }}
      style={{ border: "1px solid #555" }}
    >
      <div className="flex w-[100%] flex-row items-center justify-end">
        <IconButton
          onClick={() => {
            setIsOpened(false);
          }}
        >
          <CloseIcon />
        </IconButton>
      </div>
      {children}
    </form>
  );
};
