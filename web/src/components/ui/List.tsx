import ReactJson from "@microlink/react-json-view";
import DeleteIcon from "@mui/icons-material/Delete";
import EditIcon from "@mui/icons-material/Edit";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import type FolderIcon from "@mui/icons-material/Folder";
import OpenInNew from "@mui/icons-material/OpenInNew";
import Accordion from "@mui/material/Accordion";
import AccordionDetails from "@mui/material/AccordionDetails";
import AccordionSummary from "@mui/material/AccordionSummary";
import Avatar from "@mui/material/Avatar";
import IconButton from "@mui/material/IconButton";
import List from "@mui/material/List";
import ListItemAvatar from "@mui/material/ListItemAvatar";
import ListItemText from "@mui/material/ListItemText";

type ListItemBase = {
  long_display: string;
  text: string;
};

type ListProps<A extends ListItemBase> = {
  AvatarClass: typeof FolderIcon;
  items: A[];
  onDelete: (item: A) => void;
  onEdit: (item: A) => void;
  onOpenUrl?: (item: A) => void;
};

export const EntitiesList = <A extends ListItemBase>({
  AvatarClass,
  items,
  onDelete,
  onEdit,
  onOpenUrl,
}: ListProps<A>) => (
  <List>
    {items.map((item) => (
      <Accordion key={item.long_display}>
        <AccordionSummary
          aria-controls="panel2-content"
          expandIcon={<ExpandMoreIcon />}
          id="panel2-header"
        >
          <div className="flex w-[100%] flex-row items-center justify-between gap-[12px] pr-[24px]">
            <ListItemAvatar>
              <Avatar sx={{ height: 40, width: 40 }}>
                <AvatarClass sx={{ height: "30px", width: "30px" }} />
              </Avatar>
            </ListItemAvatar>
            <ListItemText className="flex-1" primary={item.long_display} />
            {onOpenUrl && (
              <IconButton
                aria-label="Open"
                edge="end"
                onClick={(e) => {
                  e.stopPropagation();
                  onOpenUrl(item);
                }}
              >
                <OpenInNew />
              </IconButton>
            )}
            <IconButton
              aria-label="Edit"
              edge="end"
              onClick={(e) => {
                e.stopPropagation();
                onEdit(item);
              }}
            >
              <EditIcon />
            </IconButton>
            <IconButton
              aria-label="Delete"
              edge="end"
              onClick={(e) => {
                e.stopPropagation();
                onDelete(item);
              }}
            >
              <DeleteIcon />
            </IconButton>
          </div>
        </AccordionSummary>
        <AccordionDetails>
          {(() => {
            const parsed = JSON.parse(item.text);

            return (
              <div className="border-[1px] border-[#333] p-[8px]">
                <ReactJson src={parsed} theme="monokai" />
              </div>
            );
          })()}
        </AccordionDetails>
      </Accordion>
    ))}
  </List>
);
