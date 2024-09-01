import Head from "next/head";
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
  upsertConsumer,
  upsertConsumerGroup,
  upsertRoute,
  upsertSecret,
  upsertService,
  upsertStreamRoute,
  upsertUpstream,
  deleteConsumer,
  deleteConsumerGroup,
  deleteRoute,
  deleteSecret,
  deleteService,
  deleteStreamRoute,
  deleteUpstream,
  getFileConfig,
  getHealthCheck,
  getSchema,
  reloadPlugins,
} from "../lib/client";
import {
  useFetchConsumerGroups,
  useFetchConsumers,
  useFetchRoutes,
  useFetchSecrets,
  useFetchServerInfo,
  useFetchServices,
  useFetchStreamRoutes,
  useFetchUpstreams,
} from "../lib/queries";

import type { EntityFieldsItems } from "./EntityField";
import { EntityField, parseEntityFields } from "./EntityField";
import { Form, Section, SubmitButton } from "./Section";
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
  const { serverInfo } = useFetchServerInfo();
  const { refetchRoutes, routes } = useFetchRoutes();
  const { refetchUpstreams, upstreams } = useFetchUpstreams();
  const { consumers, refetchConsumers } = useFetchConsumers();
  const { consumerGroups, refetchConsumerGroups } = useFetchConsumerGroups();
  const { refetchServices, services } = useFetchServices();
  const { refetchSecrets, secrets } = useFetchSecrets();
  const { refetchStreamRoutes, streamRoutes } = useFetchStreamRoutes();

  const [openedForms, setOpenedForms] = useState(new Set<string>());
  const [editingForms, setEditingForms] = useState(new Set<string>());

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

  const setFormOpened =
    (entity: { docs_key: string }) => (o: boolean, isEditing?: boolean) => {
      if (o) {
        setOpenedForms(new Set(openedForms).add(entity.docs_key));
      } else {
        openedForms.delete(entity.docs_key);
        setOpenedForms(new Set(openedForms));
      }

      if (isEditing) {
        if (o) {
          setEditingForms(new Set(editingForms).add(entity.docs_key));
        } else {
          editingForms.delete(entity.docs_key);
          setEditingForms(new Set(editingForms));
        }
      }
    };

  const getIsOpened = (entity: { docs_key: string }) =>
    openedForms.has(entity.docs_key);

  const getIsEditing = (entity: { docs_key: string }) =>
    editingForms.has(entity.docs_key);

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

  const handleDeleteError = (err: unknown) => {
    if (typeof err === "string") {
      setSnackbar(err);
    }

    throw err;
  };

  const prepareEdit = (entity: { text: string }) => {
    const parsed = JSON.parse(entity.text)?.value;

    Object.keys(parsed).forEach((key) => {
      if (key === "plugins") {
        parsed.plugins = Object.entries(parsed.plugins).reduce(
          (acc, [pluginName, pluginValue]) => {
            const newPluginValue = Object.entries(
              pluginValue as Record<string, unknown>,
            ).reduce(
              (acc2, [pluginOption, pluginOptionValue]) => {
                if (typeof pluginOptionValue === "object") {
                  acc2[pluginOption] = JSON.stringify(pluginOptionValue);
                } else if (typeof pluginOptionValue === "boolean") {
                  acc2[pluginOption] = pluginOptionValue.toString();
                } else {
                  acc2[pluginOption] = pluginOptionValue;
                }

                return acc2;
              },
              {
                enabled: "true",
              } as Record<string, unknown>,
            );

            acc[pluginName] = newPluginValue;

            return acc;
          },
          {} as Record<string, Record<string, unknown>>,
        );
      } else {
        if (typeof parsed[key] === "object") {
          parsed[key] = JSON.stringify(parsed[key]);
        } else if (typeof parsed[key] === "boolean") {
          parsed[key] = parsed[key].toString();
        }
      }
    });

    return parsed;
  };

  return (
    <>
      <Head>
        <title>APISIX Dashboard</title>
      </Head>
      <TopBar />
      <div className="m-auto max-w-[1024px] pb-[50px] pt-[100px]">
        {serverInfo && !serverInfo.is_standalone && (
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
                  getSchema().then(setControlData).catch(setSnackbar);
                }}
              >
                Get Schema
              </Button>
              <Button
                onClick={() => {
                  getHealthCheck().then(setControlData).catch(setSnackbar);
                }}
              >
                Get health check
              </Button>
              <Button
                onClick={() => {
                  reloadPlugins()
                    .then((r) =>
                      setSnackbar(`Response: ${(r as string) || '""'}`),
                    )
                    .catch(setSnackbar);
                }}
              >
                Reload plugins
              </Button>
              <Button
                onClick={() => {
                  getFileConfig()
                    .then((r) => {
                      setControlData(r);
                    })
                    .catch(() => {
                      setSnackbar(
                        "Failed to get file config, check apisix-admin-panel server logs",
                      );
                    });
                }}
              >
                Get file config
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
        )}
        <Section
          title={
            <h2 className={sectionTitleClass}>
              <span>Consumers{consumers ? ` (${consumers.total})` : ""}</span>
              <AdminDocs model={WasmConsumer} />
            </h2>
          }
        >
          <Form
            isOpened={getIsOpened(WasmConsumer)}
            onSubmit={({ onComplete }) => {
              const consumer = parseEntityFields(WasmConsumer, consumerFields);

              Promise.resolve()
                .then(() =>
                  upsertConsumer(consumer, getIsEditing(WasmConsumer)),
                )
                .then(() => refetchConsumers())
                .then(() => {
                  setConsumerFields(null);
                  onComplete();
                })
                .catch((err) => {
                  setSnackbar(err);
                });
            }}
            setIsOpened={setFormOpened(WasmConsumer)}
          >
            <EntityField
              entity={WasmConsumer}
              isEditing={getIsEditing(WasmConsumer)}
              items={consumerFields}
              setItems={setConsumerFields}
            />
            <SubmitButton isEditing={getIsEditing(WasmConsumer)} />
          </Form>
          {!!consumers && (
            <EntitiesList
              AvatarClass={AccountCircleIcon}
              items={consumers.list}
              onDelete={(consumer) => {
                setDialogOpts({
                  onAccept: () =>
                    deleteConsumer(consumer)
                      .then(() => refetchConsumers())
                      .catch(handleDeleteError),
                  text: `Are you sure you want to delete consumer "${consumer.short_display}"?`,
                  title: "Delete consumer",
                });
              }}
              onEdit={(consumer) => {
                setConsumerFields(prepareEdit(consumer));
                setFormOpened(WasmConsumer)(true, true);
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
            isOpened={getIsOpened(WasmConsumerGroup)}
            onSubmit={({ onComplete }) => {
              const group = parseEntityFields(
                WasmConsumerGroup,
                consumerGroupFields,
              );

              Promise.resolve()
                .then(() =>
                  upsertConsumerGroup(group, getIsEditing(WasmConsumerGroup)),
                )
                .then(() => refetchConsumerGroups())
                .then(() => {
                  setConsumerGroupFields(null);
                  onComplete();
                })
                .catch((err) => {
                  setSnackbar(err);
                });
            }}
            setIsOpened={setFormOpened(WasmConsumerGroup)}
          >
            <EntityField
              entity={WasmConsumerGroup}
              isEditing={getIsEditing(WasmConsumerGroup)}
              items={consumerGroupFields}
              setItems={setConsumerGroupFields}
            />
            <SubmitButton isEditing={getIsEditing(WasmConsumerGroup)} />
          </Form>
          {!!consumerGroups && (
            <EntitiesList
              AvatarClass={GroupIcon}
              items={consumerGroups.list}
              onDelete={(consumerGroup) => {
                setDialogOpts({
                  onAccept: () =>
                    deleteConsumerGroup(consumerGroup)
                      .then(() => refetchConsumerGroups())
                      .catch(handleDeleteError),
                  text: `Are you sure you want to delete consumer group "${consumerGroup.short_display}"?`,
                  title: "Delete consumer group",
                });
              }}
              onEdit={(consumerGroup) => {
                setConsumerGroupFields(prepareEdit(consumerGroup));
                setFormOpened(WasmConsumerGroup)(true, true);
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
            isOpened={getIsOpened(WasmRoute)}
            onSubmit={({ onComplete }) => {
              const route = parseEntityFields(WasmRoute, routeFields);

              Promise.resolve()
                .then(() => upsertRoute(route, getIsEditing(WasmRoute)))
                .then(() => refetchRoutes())
                .then(() => {
                  setRouteFields(null);
                  onComplete();
                })
                .catch((err) => {
                  setSnackbar(err);
                });
            }}
            setIsOpened={setFormOpened(WasmRoute)}
          >
            <EntityField
              entity={WasmRoute}
              isEditing={getIsEditing(WasmRoute)}
              items={routeFields}
              setItems={setRouteFields}
            />
            <SubmitButton isEditing={getIsEditing(WasmRoute)} />
          </Form>
          {!!routes && (
            <EntitiesList
              AvatarClass={AltRouteIcon}
              items={routes.list}
              onDelete={(route) => {
                setDialogOpts({
                  onAccept: () =>
                    deleteRoute(route)
                      .then(() => refetchRoutes())
                      .catch(handleDeleteError),
                  text: `Are you sure you want to delete route "${route.short_display}"?`,
                  title: "Delete route",
                });
              }}
              onEdit={(route) => {
                setRouteFields(prepareEdit(route));
                setFormOpened(WasmRoute)(true, true);
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
            isOpened={getIsOpened(WasmSecret)}
            onSubmit={({ onComplete }) => {
              const secret = parseEntityFields(WasmSecret, secretFields);

              Promise.resolve()
                .then(() => upsertSecret(secret, getIsEditing(WasmSecret)))
                .then(() => refetchSecrets())
                .then(() => {
                  setSecretFields(null);
                  onComplete();
                })
                .catch((err) => {
                  setSnackbar(err);
                });
            }}
            setIsOpened={setFormOpened(WasmSecret)}
          >
            <EntityField
              entity={WasmSecret}
              isEditing={getIsEditing(WasmSecret)}
              items={secretFields}
              setItems={setSecretFields}
            />
            <SubmitButton isEditing={getIsEditing(WasmSecret)} />
          </Form>
          {!!secrets && (
            <EntitiesList
              AvatarClass={GroupIcon}
              items={secrets.list}
              onDelete={(secret) => {
                setDialogOpts({
                  onAccept: () =>
                    deleteSecret(secret)
                      .then(() => refetchSecrets())
                      .catch(handleDeleteError),
                  text: `Are you sure you want to delete secret "${secret.short_display}"?`,
                  title: "Delete secret",
                });
              }}
              onEdit={(secret) => {
                setSecretFields(prepareEdit(secret));
                setFormOpened(WasmSecret)(true, true);
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
            isOpened={getIsOpened(WasmService)}
            onSubmit={({ onComplete }) => {
              const service = parseEntityFields(WasmService, serviceFields);

              Promise.resolve()
                .then(() => upsertService(service, getIsEditing(WasmService)))
                .then(() => setServiceFields(null))
                .then(() => {
                  refetchServices();
                  onComplete();
                })
                .catch((err) => {
                  setSnackbar(err);
                });
            }}
            setIsOpened={setFormOpened(WasmService)}
          >
            <EntityField
              entity={WasmService}
              isEditing={getIsEditing(WasmService)}
              items={serviceFields}
              setItems={setServiceFields}
            />
            <SubmitButton isEditing={getIsEditing(WasmService)} />
          </Form>
          {!!services && (
            <EntitiesList
              AvatarClass={LanIcon}
              items={services.list}
              onDelete={(service) => {
                setDialogOpts({
                  onAccept: () =>
                    deleteService(service)
                      .then(() => refetchServices())
                      .catch(handleDeleteError),
                  text: `Are you sure you want to delete service "${
                    service.short_display
                  }"?`,
                  title: "Delete service",
                });
              }}
              onEdit={(service) => {
                setServiceFields(prepareEdit(service));
                setFormOpened(WasmService)(true, true);
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
            isOpened={getIsOpened(WasmStreamRoute)}
            onSubmit={({ onComplete }) => {
              const streamRoute = parseEntityFields(
                WasmStreamRoute,
                streamRouteFields,
              );

              Promise.resolve()
                .then(() =>
                  upsertStreamRoute(streamRoute, getIsEditing(WasmStreamRoute)),
                )
                .then(() => refetchStreamRoutes())
                .then(() => {
                  setStreamRouteFields(null);
                  onComplete();
                })
                .catch((err) => {
                  setSnackbar(err);
                });
            }}
            setIsOpened={setFormOpened(WasmStreamRoute)}
          >
            <EntityField
              entity={WasmStreamRoute}
              isEditing={getIsEditing(WasmStreamRoute)}
              items={streamRouteFields}
              setItems={setStreamRouteFields}
            />
            <SubmitButton isEditing={getIsEditing(WasmStreamRoute)} />
          </Form>
          {!!streamRoutes && (
            <EntitiesList
              AvatarClass={AccountCircleIcon}
              items={streamRoutes.list}
              onDelete={(streamRoute) => {
                setDialogOpts({
                  onAccept: () =>
                    deleteStreamRoute(streamRoute)
                      .then(() => refetchStreamRoutes())
                      .catch(handleDeleteError),
                  text: `Are you sure you want to delete stream route "${streamRoute.short_display}"?`,
                  title: "Delete stream route",
                });
              }}
              onEdit={(streamRoute) => {
                setStreamRouteFields(prepareEdit(streamRoute));
                setFormOpened(WasmStreamRoute)(true, true);
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
            isOpened={getIsOpened(WasmUpstream)}
            onSubmit={({ onComplete }) => {
              const upstream = parseEntityFields(WasmUpstream, upstreamFields);

              Promise.resolve()
                .then(() =>
                  upsertUpstream(upstream, getIsEditing(WasmUpstream)),
                )
                .then(() => refetchUpstreams())
                .then(() => {
                  setUpstreamFields(null);
                  onComplete();
                })
                .catch((err) => {
                  setSnackbar(err);
                });
            }}
            setIsOpened={setFormOpened(WasmUpstream)}
          >
            <EntityField
              entity={WasmUpstream}
              isEditing={getIsEditing(WasmUpstream)}
              items={upstreamFields}
              setItems={setUpstreamFields}
            />
            <SubmitButton isEditing={getIsEditing(WasmUpstream)} />
          </Form>
          {!!upstreams && (
            <EntitiesList
              AvatarClass={SettingsInputAntennaIcon}
              items={upstreams.list}
              onDelete={(upstream) => {
                setDialogOpts({
                  onAccept: () =>
                    deleteUpstream(upstream)
                      .then(() => refetchUpstreams())
                      .catch(handleDeleteError),
                  text: `Are you sure you want to delete upstream "${upstream.short_display}"?`,
                  title: "Delete upstream",
                });
              }}
              onEdit={(upstream) => {
                setUpstreamFields(prepareEdit(upstream));
                setFormOpened(WasmUpstream)(true, true);
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
