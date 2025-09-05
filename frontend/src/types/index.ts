export interface IUser {
  created_at: string;
  email: string;
  first_name: string;
  id: number;
  last_login_at: string | null;
  last_name: string;
  phone: string;
  role: "user" | "admin";
  updated_at: string;
}
