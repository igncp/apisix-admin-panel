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

const createMethod =
  <
    A extends {
      create_response: (r: unknown) => void;
      new (): {
        create: () => WasmProxyFetchOpts;
      };
    },
  >(
    c: A,
  ) =>
  (item: InstanceType<A>) =>
    Promise.resolve()
      .then(() => fetchApisixAdmin(item.create()))
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

export const createConsumer = createMethod(WasmConsumer);
export const createConsumerGroup = createMethod(WasmConsumerGroup);
export const createRoute = createMethod(WasmRoute);
export const createSecret = createMethod(WasmSecret);
export const createService = createMethod(WasmService);
export const createStreamRoute = createMethod(WasmStreamRoute);
export const createUpstream = createMethod(WasmUpstream);

export const getSchema = () => fetchApisixControl(WasmControlPane.get_schema());
export const getHealthCheck = () =>
  fetchApisixControl(WasmControlPane.get_health_check());
export const reloadPlugins = () =>
  fetchApisixControl(WasmControlPane.reload_plugins());
