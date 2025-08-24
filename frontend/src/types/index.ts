declare module "@mui/material/styles" {
  interface Palette {
    neutralInfo: Palette["primary"];
  }
  interface PaletteOptions {
    neutralInfo?: PaletteOptions["primary"];
  }
  interface TypographyVariants {
    p1: React.CSSProperties;
  }
  interface TypographyVariantsOptions {
    p1?: React.CSSProperties;
  }
}

declare module "@mui/material/Typography" {
  interface TypographyPropsVariantOverrides {
    p1: true;
  }
}

export interface IUser {
  email: string;
  first_name: string;
  id: number;
  last_name: string;
  phone: string;
}
