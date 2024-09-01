import type { WasmProxyFetchOpts } from "pkg";
import {
  WasmConsumer,
  WasmConsumerGroup,
  WasmControlPane,
  WasmRoute,
  WasmSecret,
  WasmService,
  WasmStreamRoute,
  WasmUpstream,
} from "pkg";
import type { ServerInfo } from "src/bindings/ServerInfo";

const baseUrl =
  process.env.NODE_ENV === "production" ? "" : "http://localhost:9000";

const fetchApisixAdmin = async <T>(body: WasmProxyFetchOpts) =>
  await fetch(`${baseUrl}/api/apisix-admin`, {
    body: body.format(),
    headers: {
      "Content-Type": "application/json",
    },
    method: "POST",
  }).then((res) => res.json() as T);

const fetchApisixControl = async <T>(body: WasmProxyFetchOpts) =>
  await fetch(`${baseUrl}/api/apisix-control`, {
    body: body.format(),
    headers: {
      "Content-Type": "application/json",
    },
    method: "POST",
  }).then((res) => res.json() as T);

const genGetMethod =
  <
    A extends {
      get_all: () => WasmProxyFetchOpts;
      get_all_response: (res: unknown) => ReturnType<A["get_all_response"]>;
    },
  >(
    c: A,
  ) =>
  async () =>
    fetchApisixAdmin(c.get_all()).then((res) => c.get_all_response(res));

const deleteMethod =
  <
    A extends {
      delete_response: (r: unknown) => void;
      new (): {
        delete: () => WasmProxyFetchOpts;
      };
    },
  >(
    c: A,
  ) =>
  (item: InstanceType<A>) =>
    Promise.resolve()
      .then(() => fetchApisixAdmin(item.delete()))
      .then((r) => c.delete_response(r));

const upsertMethod =
  <
    A extends {
      create_response: (r: unknown) => void;
      new (): {
        create: () => WasmProxyFetchOpts;
        update: () => WasmProxyFetchOpts;
      };
    },
  >(
    c: A,
  ) =>
  (item: InstanceType<A>, isEditing: boolean) =>
    Promise.resolve()
      .then(() => fetchApisixAdmin(isEditing ? item.update() : item.create()))
      .then((r) => c.create_response(r));

export const getConsumerGroups = genGetMethod(WasmConsumerGroup);
export const getConsumers = genGetMethod(WasmConsumer);
export const getRoutes = genGetMethod(WasmRoute);
export const getSecrets = genGetMethod(WasmSecret);
export const getServices = genGetMethod(WasmService);
export const getStreamRoute = genGetMethod(WasmStreamRoute);
export const getUpstreams = genGetMethod(WasmUpstream);

export const deleteConsumer = deleteMethod(WasmConsumer);
export const deleteConsumerGroup = deleteMethod(WasmConsumerGroup);
export const deleteRoute = deleteMethod(WasmRoute);
export const deleteSecret = deleteMethod(WasmSecret);
export const deleteService = deleteMethod(WasmService);
export const deleteStreamRoute = deleteMethod(WasmStreamRoute);
export const deleteUpstream = deleteMethod(WasmUpstream);

export const upsertConsumer = upsertMethod(WasmConsumer);
export const upsertConsumerGroup = upsertMethod(WasmConsumerGroup);
export const upsertRoute = upsertMethod(WasmRoute);
export const upsertSecret = upsertMethod(WasmSecret);
export const upsertService = upsertMethod(WasmService);
export const upsertStreamRoute = upsertMethod(WasmStreamRoute);
export const upsertUpstream = upsertMethod(WasmUpstream);

export const getSchema = () => fetchApisixControl(WasmControlPane.get_schema());
export const getHealthCheck = () =>
  fetchApisixControl(WasmControlPane.get_health_check());
export const reloadPlugins = () =>
  fetchApisixControl(WasmControlPane.reload_plugins());
export const getFileConfig = () =>
  fetch(`${baseUrl}/api/apisix-config`, {
    headers: {
      "Content-Type": "application/json",
    },
  }).then((res) => res.json() as unknown);

export const getServerInfo = async (): Promise<ServerInfo> =>
  fetch(`${baseUrl}/api/info`, {
    headers: {
      "Content-Type": "application/json",
    },
  }).then((res) => res.json() as unknown as ServerInfo);
