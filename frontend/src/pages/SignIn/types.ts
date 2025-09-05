import type { IUser } from "../../types";

export interface IFormData {
  email: string;
  password: string;
  remember: boolean;
}

export interface Errors {
  email?: string;
  password?: string;
}

export interface ServerValidationErrors {
  errors: {
    email?: string;
    password?: string;
    [key: string]: string | undefined;
  };
}

export interface AuthResponse {
  token: string;
  user: IUser
}

