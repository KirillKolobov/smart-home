export interface IFormData {
  first_name: string;
  last_name: string;
  phone: string;
  email: string;
  password: string;
  repeat_password: string;
  accept_privacy_policy: boolean;
}

export interface Errors {
  first_name?: string;
  last_name?: string;
  email?: string;
  password?: string;
  phone?: string;
}

export interface ServerValidationErrors {
  errors: {
    first_name?: string;
    last_name?: string;
    email?: string;
    password?: string;
    phone?: string;
    [key: string]: string | undefined;
  };
}
