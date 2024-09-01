import { useQuery } from "@tanstack/react-query";

import {
  getConsumers,
  getRoutes,
  getConsumerGroups,
  getServices,
  getUpstreams,
  getSecrets,
  getStreamRoute,
  getServerInfo,
} from "./client";

export const useFetchConsumers = () => {
  const { data, refetch, ...query } = useQuery({
    queryFn: getConsumers,
    queryKey: ["consumers"],
  });

  return {
    ...query,
    consumers: data,
    refetchConsumers: refetch,
  };
};

export const useFetchConsumerGroups = () => {
  const { data, refetch, ...query } = useQuery({
    queryFn: getConsumerGroups,
    queryKey: ["consumerGroups"],
  });

  return {
    ...query,
    consumerGroups: data,
    refetchConsumerGroups: refetch,
  };
};

export const useFetchRoutes = () => {
  const { data, refetch, ...query } = useQuery({
    queryFn: getRoutes,
    queryKey: ["routes"],
  });

  return {
    ...query,
    refetchRoutes: refetch,
    routes: data,
  };
};

export const useFetchServices = () => {
  const { data, refetch, ...query } = useQuery({
    queryFn: getServices,
    queryKey: ["services"],
  });

  return {
    ...query,
    refetchServices: refetch,
    services: data,
  };
};

export const useFetchUpstreams = () => {
  const { data, refetch, ...query } = useQuery({
    queryFn: getUpstreams,
    queryKey: ["upstreams"],
  });

  return {
    ...query,
    refetchUpstreams: refetch,
    upstreams: data,
  };
};

export const useFetchSecrets = () => {
  const { data, refetch, ...query } = useQuery({
    queryFn: getSecrets,
    queryKey: ["secrets"],
  });

  return {
    ...query,
    refetchSecrets: refetch,
    secrets: data,
  };
};

export const useFetchStreamRoutes = () => {
  const { data, refetch, ...query } = useQuery({
    queryFn: getStreamRoute,
    queryKey: ["streamRoutes"],
  });

  return {
    ...query,
    refetchStreamRoutes: refetch,
    streamRoutes: data,
  };
};

export const useFetchServerInfo = () => {
  const { data, refetch, ...query } = useQuery({
    queryFn: getServerInfo,
    queryKey: ["serverInfo"],
  });

  return {
    ...query,
    refetchServerInfo: refetch,
    serverInfo: data,
  };
};
