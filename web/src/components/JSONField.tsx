// @ts-expect-error Missing type definitions for JSONEditor
import JSONEditor from "jsoneditor";
import { useEffect, useRef } from "react";

type Props = {
  content: string;
  name: string;
  setContentWrap: {
    fn: (val: string) => void;
  };
};

export const JSONField = ({ content, name, setContentWrap }: Props) => {
  const itemId = useRef<null | string>(null);
  const initialContent = useRef<null | string>(null);

  if (initialContent.current === null) {
    initialContent.current = content;
  }

  if (itemId.current === null) {
    itemId.current = Math.random().toString(36).substring(7);
  }

  const itemIdParsed = `json-field-${itemId.current}`;
  const { current: initialContentStr } = initialContent;

  useEffect(() => {
    // https://github.com/josdejong/jsoneditor/blob/develop/src/js/JSONEditor.js
    // Search for: VALID_OPTIONS
    const options = {
      autocomplete: true,
      mainMenuBar: false,
      mode: "code",
      navigationBar: false,
      onChangeText: (v: string) => {
        setContentWrap.fn(v);
      },
      statusBar: false,
    };

    const container = document.getElementById(itemIdParsed);
    const editor = new JSONEditor(container, options);

    const initialJson = (() => {
      try {
        return JSON.parse(initialContentStr);
      } catch {
        return {};
      }
    })();

    editor.set(initialJson);

    return () => {
      editor.destroy();
    };
  }, [initialContentStr, itemIdParsed, setContentWrap]);

  return (
    <div>
      <div>{name}:</div>
      <div id={itemIdParsed} />
    </div>
  );
};
