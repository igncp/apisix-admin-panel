import {
  WasmConsumer,
  WasmConsumerGroup,
  WasmRoute,
  WasmSecret,
  WasmService,
  WasmStreamRoute,
  WasmUpstream,
} from "pkg";
import { useEffect, useRef, useState } from "react";

import ReactJson from "@microlink/react-json-view";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import Snackbar from "@mui/material/Snackbar";

import {
  createConsumer,
  createConsumerGroup,
  createRoute,
  createSecret,
  createService,
  createStreamRoute,
  createUpstream,
  deleteConsumer,
  deleteConsumerGroup,
  deleteRoute,
  deleteSecret,
  deleteService,
  deleteStreamRoute,
  deleteUpstream,
  getHealthCheck,
  getSchema,
} from "../lib/client";
import {
  useFetchConsumerGroups,
  useFetchConsumers,
  useFetchRoutes,
  useFetchSecrets,
  useFetchServices,
  useFetchStreamRoutes,
  useFetchUpstreams,
} from "../lib/queries";

import type { EntityFieldsItems } from "./EntityField";
import { EntityField, parseEntityFields } from "./EntityField";
import { Form, Section } from "./Section";
import Button from "./ui/Button";
import { EntitiesList } from "./ui/List";
import TopBar from "./ui/TopBar";
import {
  AccountCircleIcon,
  AltRouteIcon,
  DeleteIcon,
  GroupIcon,
  IconButton,
  LanIcon,
  SettingsInputAntennaIcon,
} from "./ui/icons/Icons";

type DocsLinkProps = {
  href: string;
};

const DocsLink = ({ href }: DocsLinkProps) => (
  <small>
    <a
      className="text-blue-200"
      href={href}
      onClick={(e) => {
        e.stopPropagation();
      }}
      rel="noreferrer"
      target="_blank"
    >
      Docs
    </a>
  </small>
);

type AdminDocsProps = {
  model: { docs_key: string };
};

const AdminDocs = ({ model }: AdminDocsProps) => (
  <DocsLink
    href={`https://apisix.apache.org/docs/apisix/admin-api/#${model.docs_key}`}
  />
);

const sectionTitleClass =
  "flex flex-row items-baseline justify-between w-full pr-[12px]";

export default function Dashboard() {
  const { refetchRoutes, routes } = useFetchRoutes();
  const { refetchUpstreams, upstreams } = useFetchUpstreams();
  const { consumers, refetchConsumers } = useFetchConsumers();
  const { consumerGroups, refetchConsumerGroups } = useFetchConsumerGroups();
  const { refetchServices, services } = useFetchServices();
  const { refetchSecrets, secrets } = useFetchSecrets();
  const { refetchStreamRoutes, streamRoutes } = useFetchStreamRoutes();

  const snackbarTimeout = useRef<null | number>(null);

  const [controlData, setControlData] = useState<unknown>(null);

  const [consumerFields, setConsumerFields] = useState<EntityFieldsItems>(null);
  const [routeFields, setRouteFields] = useState<EntityFieldsItems>(null);
  const [serviceFields, setServiceFields] = useState<EntityFieldsItems>(null);
  const [secretFields, setSecretFields] = useState<EntityFieldsItems>(null);
  const [upstreamFields, setUpstreamFields] = useState<EntityFieldsItems>(null);

  const [consumerGroupFields, setConsumerGroupFields] =
    useState<EntityFieldsItems>(null);

  const [snackbarMessage, setSnackbarMessage] = useState<null | string>(null);
  const [snackbarOpen, setSnackbarOpen] = useState(false);

  const [rendered, setRendered] = useState(false);

  const [streamRouteFields, setStreamRouteFields] =
    useState<EntityFieldsItems>(null);

  const [dialogOpts, setDialogOpts] = useState<{
    onAccept: () => Promise<unknown>;
    text: string;
    title: string;
  } | null>(null);

  useEffect(() => {
    setRendered(true);
  }, []);

  if (!rendered) {
    return null;
  }

  const setSnackbar = (message: string) => {
    setSnackbarMessage(message);
    setSnackbarOpen(true);

    if (snackbarTimeout.current) {
      clearTimeout(snackbarTimeout.current);
    }

    snackbarTimeout.current = window.setTimeout(() => {
      setSnackbarOpen(false);
      snackbarTimeout.current = null;
    }, 3000);
  };

  return (
    <>
      <TopBar />
      <div className="m-auto max-w-[1024px] pb-[50px] pt-[100px]">
        <Section
          title={
            <h2 className={sectionTitleClass}>
              <span>Control Pane</span>
              <DocsLink href="https://apisix.apache.org/docs/apisix/control-api" />
            </h2>
          }
        >
          <div className="flex flex-row gap-[12px]">
            <Button
              onClick={() => {
                getSchema().then(setControlData);
              }}
            >
              Get Schema
            </Button>
            <Button
              onClick={() => {
                getHealthCheck().then(setControlData);
              }}
            >
              Get health check
            </Button>
          </div>
          {!!controlData && (
            <div>
              <IconButton
                aria-label="Clear"
                edge="end"
                onClick={() => setControlData(null)}
              >
                <DeleteIcon />
              </IconButton>
              <ReactJson src={controlData} theme="monokai" />
            </div>
          )}
        </Section>
        <Section
          title={
            <h2 className={sectionTitleClass}>
              <span>Consumers{consumers ? ` (${consumers.total})` : ""}</span>
              <AdminDocs model={WasmConsumer} />
            </h2>
          }
        >
          <Form
            onSubmit={({ onComplete }) => {
              Promise.resolve()
                .then(() => {
                  const consumer = parseEntityFields(
                    WasmConsumer,
                    consumerFields,
                  );

                  return createConsumer(consumer);
                })
                .then(() => refetchConsumers())
                .then(() => {
                  setConsumerFields(null);
                  onComplete();
                })
                .catch((err) => {
                  setSnackbar(err);
                });
            }}
          >
            <EntityField
              entity={WasmConsumer}
              items={consumerFields}
              setItems={setConsumerFields}
            />
            <Button type="submit">Create</Button>
          </Form>
          {!!consumers && (
            <EntitiesList
              AvatarClass={AccountCircleIcon}
              items={consumers.list}
              onDelete={(consumer) => {
                setDialogOpts({
                  onAccept: () =>
                    deleteConsumer(consumer).then(() => refetchConsumers()),
                  text: `Are you sure you want to delete consumer "${consumer.short_display}"?`,
                  title: "Delete consumer",
                });
              }}
            />
          )}
        </Section>
        <Section
          title={
            <h2 className={sectionTitleClass}>
              <span>
                Consumer Groups
                {consumerGroups ? ` (${consumerGroups.total})` : ""}
              </span>
              <AdminDocs model={WasmConsumerGroup} />
            </h2>
          }
        >
          <Form
            onSubmit={() => {
              const group = parseEntityFields(
                WasmConsumerGroup,
                consumerGroupFields,
              );

              createConsumerGroup(group)
                .then(() => refetchConsumerGroups())
                .then(() => {
                  setConsumerGroupFields(null);
                })
                .catch((err) => {
                  setSnackbar(err);
                });
            }}
          >
            <EntityField
              entity={WasmConsumerGroup}
              items={consumerGroupFields}
              setItems={setConsumerGroupFields}
            />
            <Button type="submit">Create</Button>
          </Form>
          {!!consumerGroups && (
            <EntitiesList
              AvatarClass={GroupIcon}
              items={consumerGroups.list}
              onDelete={(consumerGroup) => {
                setDialogOpts({
                  onAccept: () =>
                    deleteConsumerGroup(consumerGroup).then(() =>
                      refetchConsumerGroups(),
                    ),
                  text: `Are you sure you want to delete consumer group "${consumerGroup.short_display}"?`,
                  title: "Delete consumer group",
                });
              }}
            />
          )}
        </Section>
        <Section
          title={
            <h2 className={sectionTitleClass}>
              <span>Routes{routes ? ` (${routes.total})` : ""}</span>
              <AdminDocs model={WasmRoute} />
            </h2>
          }
        >
          <Form
            onSubmit={() => {
              Promise.resolve()
                .then(() => {
                  const route = parseEntityFields(WasmRoute, routeFields);

                  return createRoute(route);
                })
                .then(() => refetchRoutes())
                .then(() => {
                  setRouteFields(null);
                })
                .catch((err) => {
                  setSnackbar(err);
                });
            }}
          >
            <EntityField
              entity={WasmRoute}
              items={routeFields}
              setItems={setRouteFields}
            />
            <Button type="submit">Create</Button>
          </Form>
          {!!routes && (
            <EntitiesList
              AvatarClass={AltRouteIcon}
              items={routes.list}
              onDelete={(route) => {
                setDialogOpts({
                  onAccept: () =>
                    deleteRoute(route).then(() => refetchRoutes()),
                  text: `Are you sure you want to delete route "${route.short_display}"?`,
                  title: "Delete route",
                });
              }}
            />
          )}
        </Section>
        <Section
          title={
            <h2 className={sectionTitleClass}>
              <span>Secrets{secrets ? ` (${secrets.total})` : ""}</span>
              <AdminDocs model={WasmUpstream} />
            </h2>
          }
        >
          <Form
            onSubmit={() => {
              const secret = parseEntityFields(WasmSecret, secretFields);

              createSecret(secret)
                .then(() => refetchSecrets())
                .then(() => {
                  setSecretFields(null);
                })
                .catch((err) => {
                  setSnackbar(err);
                });
            }}
          >
            <EntityField
              entity={WasmSecret}
              items={secretFields}
              setItems={setSecretFields}
            />
            <Button type="submit">Create</Button>
          </Form>
          {!!secrets && (
            <EntitiesList
              AvatarClass={GroupIcon}
              items={secrets.list}
              onDelete={(secret) => {
                setDialogOpts({
                  onAccept: () =>
                    deleteSecret(secret).then(() => refetchSecrets()),
                  text: `Are you sure you want to delete secret "${secret.short_display}"?`,
                  title: "Delete secret",
                });
              }}
            />
          )}
        </Section>
        <Section
          title={
            <h2 className={sectionTitleClass}>
              <span>Services{services ? ` (${services.total})` : ""}</span>
              <AdminDocs model={WasmService} />
            </h2>
          }
        >
          <Form
            onSubmit={() => {
              const service = parseEntityFields(WasmService, serviceFields);

              Promise.resolve()
                .then(() => createService(service))
                .then(() => {
                  setServiceFields(null);
                })
                .then(() => refetchServices())
                .catch((err) => {
                  setSnackbar(err);
                });
            }}
          >
            <EntityField
              entity={WasmService}
              items={serviceFields}
              setItems={setServiceFields}
            />
            <Button type="submit">Create</Button>
          </Form>
          {!!services && (
            <EntitiesList
              AvatarClass={LanIcon}
              items={services.list}
              onDelete={(service) => {
                setDialogOpts({
                  onAccept: () =>
                    deleteService(service).then(() => refetchServices()),
                  text: `Are you sure you want to delete service "${
                    service.short_display
                  }"?`,
                  title: "Delete service",
                });
              }}
            />
          )}
        </Section>
        <Section
          title={
            <h2 className={sectionTitleClass}>
              <span>
                Stream Routes{streamRoutes ? ` (${streamRoutes.total})` : ""}
              </span>
              <AdminDocs model={WasmStreamRoute} />
            </h2>
          }
        >
          <Form
            onSubmit={() => {
              const streamRoute = parseEntityFields(
                WasmStreamRoute,
                streamRouteFields,
              );

              createStreamRoute(streamRoute)
                .then(() => refetchStreamRoutes())
                .then(() => {
                  setStreamRouteFields(null);
                })
                .catch((err) => {
                  setSnackbar(err);
                });
            }}
          >
            <EntityField
              entity={WasmStreamRoute}
              items={streamRouteFields}
              setItems={setStreamRouteFields}
            />
            <Button type="submit">Create</Button>
          </Form>
          {!!streamRoutes && (
            <EntitiesList
              AvatarClass={AccountCircleIcon}
              items={streamRoutes.list}
              onDelete={(streamRoute) => {
                setDialogOpts({
                  onAccept: () =>
                    deleteStreamRoute(streamRoute).then(() =>
                      refetchStreamRoutes(),
                    ),
                  text: `Are you sure you want to delete stream route "${streamRoute.short_display}"?`,
                  title: "Delete stream route",
                });
              }}
            />
          )}
        </Section>
        <Section
          title={
            <h2 className={sectionTitleClass}>
              <span>Upstreams{upstreams ? ` (${upstreams.total})` : ""}</span>
              <AdminDocs model={WasmUpstream} />
            </h2>
          }
        >
          <Form
            onSubmit={() => {
              const upstream = parseEntityFields(WasmUpstream, upstreamFields);

              createUpstream(upstream)
                .then(() => refetchUpstreams())
                .then(() => {
                  setUpstreamFields(null);
                })
                .catch((err) => {
                  setSnackbar(err);
                });
            }}
          >
            <EntityField
              entity={WasmUpstream}
              items={upstreamFields}
              setItems={setUpstreamFields}
            />
            <Button type="submit">Create</Button>
          </Form>
          {!!upstreams && (
            <EntitiesList
              AvatarClass={SettingsInputAntennaIcon}
              items={upstreams.list}
              onDelete={(upstream) => {
                setDialogOpts({
                  onAccept: () =>
                    deleteUpstream(upstream).then(() => refetchUpstreams()),
                  text: `Are you sure you want to delete upstream "${upstream.short_display}"?`,
                  title: "Delete upstream",
                });
              }}
            />
          )}
        </Section>
        <Snackbar
          anchorOrigin={{ horizontal: "center", vertical: "top" }}
          key="top"
          message={snackbarMessage}
          onClose={() => setSnackbarOpen(false)}
          open={snackbarOpen}
        />
        <Dialog
          aria-describedby="alert-dialog-description"
          aria-labelledby="alert-dialog-title"
          onClose={() => {
            setDialogOpts(null);
          }}
          open={!!dialogOpts}
        >
          <DialogTitle id="alert-dialog-title">{dialogOpts?.title}</DialogTitle>
          {dialogOpts?.text && (
            <DialogContent>
              <DialogContentText id="alert-dialog-description">
                {dialogOpts.text}
              </DialogContentText>
            </DialogContent>
          )}
          <DialogActions>
            <Button
              onClick={() => {
                setDialogOpts(null);
              }}
            >
              Cancel
            </Button>
            <Button
              autoFocus
              onClick={() => {
                dialogOpts?.onAccept().then(() => {
                  setDialogOpts(null);
                });
              }}
            >
              Accept
            </Button>
          </DialogActions>
        </Dialog>
      </div>
    </>
  );
}
