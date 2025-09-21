import { Navigate } from "react-router";
import { useGetUser } from "../../hooks/useGetUser";

export const Authorized = ({ children }: { children: React.ReactNode }) => {
  const { data: user, isLoading } = useGetUser();

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (user) {
    return <>{children}</>;
  }

  return <Navigate to="/sign-in" replace />;
};
