import { useQuery } from "@tanstack/react-query";
import { getToken } from "../../utils/localStorage";
import { fetchUser } from "./utils";

export const useGetUser = () => {
  return useQuery({
    queryKey: ["user"],
    queryFn: fetchUser,
    enabled: !!getToken(),
  });
};
