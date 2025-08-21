import { createTheme } from "@mui/material";

// export const theme = createTheme({ cssVariables: true });

export const theme = createTheme({
  cssVariables: true,
  palette: {
    mode: "dark",
    primary: {
      main: "#14b8a6", // teal-500
      light: "#2dd4bf", // teal-400
      dark: "#0f766e", // teal-700
      contrastText: "#ffffff",
    },
    secondary: {
      main: "#3b82f6", // blue-500
      light: "#60a5fa", // blue-400
      dark: "#1d4ed8", // blue-700
      contrastText: "#ffffff",
    },
    error: {
      main: "#ef4444", // red-500
      light: "#f87171", // red-400
      dark: "#dc2626", // red-600
    },
    warning: {
      main: "#f59e0b", // amber-500
      light: "#fbbf24", // amber-400
      dark: "#d97706", // amber-600
    },
    info: {
      main: "#3b82f6", // blue-500
      light: "#60a5fa", // blue-400
      dark: "#1e40af", // blue-800
    },
    success: {
      main: "#10b981", // emerald-500
      light: "#34d399", // emerald-400
      dark: "#059669", // emerald-600
    },
    background: {
      default: "#111827", // gray-900 - основной фон
      paper: "#1f2937", // gray-800 - фон карточек и модалов
    },
    text: {
      primary: "#ffffff", // белый текст
      secondary: "#9ca3af", // gray-400 - вторичный текст
      disabled: "#6b7280", // gray-500
    },
    divider: "#374151", // gray-700
    action: {
      active: "#14b8a6", // teal-500
      hover: "rgba(20, 184, 166, 0.08)", // teal с прозрачностью
      selected: "rgba(20, 184, 166, 0.16)",
      disabled: "#6b7280", // gray-500
      disabledBackground: "#374151", // gray-700
    },
  },
  typography: {
    // Обновленный fontFamily с Inter в приоритете
    fontFamily: [
      "ui-sans-serif",
      "Inter",
      "-apple-system",
      "BlinkMacSystemFont",
      '"Segoe UI"',
      '"Helvetica Neue"',
      "Arial",
      "sans-serif",
    ].join(","),
    // Обновленные веса шрифтов для соответствия макету
    h1: {
      fontSize: "2.25rem",
      fontWeight: 700, // Более легкий вес
      lineHeight: 1.2,
      color: "#ffffff",
      letterSpacing: "-0.025em", // Тайтер межбуквенный интервал
    },
    h2: {
      fontSize: "1.875rem",
      fontWeight: 500,
      lineHeight: 1.3,
      color: "#ffffff",
      letterSpacing: "-0.025em",
    },
    h3: {
      fontSize: "1.5rem",
      fontWeight: 500,
      lineHeight: 1.4,
      color: "#ffffff",
      letterSpacing: "-0.025em",
    },
    h4: {
      fontSize: "1.25rem",
      fontWeight: 500,
      lineHeight: 1.4,
      color: "#ffffff",
    },
    h5: {
      fontSize: "1.125rem",
      fontWeight: 500,
      lineHeight: 1.5,
      color: "#ffffff",
    },
    h6: {
      fontSize: "1rem",
      fontWeight: 500,
      lineHeight: 1.5,
      color: "#ffffff",
    },
    body1: {
      fontSize: "1rem",
      fontWeight: 400,
      lineHeight: 1.6,
      color: "#ffffff",
    },
    body2: {
      fontSize: "0.875rem",
      fontWeight: 400,
      lineHeight: 1.5,
      color: "#9ca3af",
    },
    caption: {
      fontSize: "0.75rem",
      fontWeight: 400,
      lineHeight: 1.4,
      color: "#9ca3af",
    },
    button: {
      fontWeight: 500, // Средний вес для кнопок
      textTransform: "none",
      letterSpacing: "0.025em",
    },
  },
  shape: {
    borderRadius: 8,
  },
  components: {
    // Кнопки с обновленной типографикой
    MuiButton: {
      styleOverrides: {
        root: {
          textTransform: "none",
          borderRadius: "8px",
          fontWeight: 500, // Соответствует макету
          padding: "12px 24px", // Немного больше padding как в макете
          fontSize: "1rem",
          transition: "all 0.2s ease-in-out",
        },
        contained: {
          boxShadow: "0 4px 6px -1px rgba(0, 0, 0, 0.1)",
          "&:hover": {
            boxShadow: "0 10px 15px -3px rgba(0, 0, 0, 0.1)",
            transform: "translateY(-1px)",
          },
        },
        outlined: {
          borderColor: "#374151",
          "&:hover": {
            borderColor: "#14b8a6",
            backgroundColor: "rgba(20, 184, 166, 0.08)",
          },
        },
      },
    },

    // Карточки
    MuiCard: {
      styleOverrides: {
        root: {
          backgroundColor: "#1f2937",
          borderRadius: "12px",
          border: "1px solid #374151",
          boxShadow: "0 4px 6px -1px rgba(0, 0, 0, 0.1)",
          transition: "all 0.2s ease-in-out",
          "&:hover": {
            boxShadow: "0 10px 15px -3px rgba(0, 0, 0, 0.1)",
            transform: "translateY(-2px)",
          },
        },
      },
    },

    // Поля ввода с обновленной типографикой
    MuiTextField: {
      styleOverrides: {
        root: {
          "& .MuiOutlinedInput-root": {
            backgroundColor: "#374151",
            borderRadius: "8px",
            fontFamily: "Inter, -apple-system, sans-serif",
            "& fieldset": {
              borderColor: "#4b5563",
            },
            "&:hover fieldset": {
              borderColor: "#14b8a6",
            },
            "&.Mui-focused fieldset": {
              borderColor: "#14b8a6",
              borderWidth: "2px",
            },
          },
          "& .MuiInputLabel-root": {
            color: "#9ca3af",
            fontFamily: "Inter, -apple-system, sans-serif",
            fontWeight: 400,
            "&.Mui-focused": {
              color: "#14b8a6",
            },
          },
          "& .MuiOutlinedInput-input": {
            color: "#ffffff",
            fontFamily: "Inter, -apple-system, sans-serif",
            fontWeight: 400,
            "&::placeholder": {
              color: "#6b7280",
              opacity: 1,
              fontFamily: "Inter, -apple-system, sans-serif",
            },
          },
        },
      },
    },

    // Остальные компоненты...
    MuiSwitch: {
      styleOverrides: {
        root: {
          "& .MuiSwitch-switchBase.Mui-checked": {
            color: "#14b8a6",
            "& + .MuiSwitch-track": {
              backgroundColor: "#14b8a6",
            },
          },
        },
        track: {
          backgroundColor: "#4b5563",
        },
      },
    },

    // Вкладки
    MuiTabs: {
      styleOverrides: {
        root: {
          borderBottom: "1px solid #374151",
        },
        indicator: {
          backgroundColor: "#14b8a6",
        },
      },
    },

    MuiTab: {
      styleOverrides: {
        root: {
          textTransform: "none",
          fontWeight: 500,
          fontFamily: "Inter, -apple-system, sans-serif",
          color: "#9ca3af",
          "&.Mui-selected": {
            color: "#14b8a6",
          },
        },
      },
    },
  },
});

// export const theme = createTheme({
//   colorSchemes: {
//     light: {
//       palette: {
//         primary: {
//           main: "rgb(20, 184, 166)",
//         },
//         info: {
//           main: "#fff",
//         },
//         neutralInfo: {
//           main: "rgb(229, 231, 235)",
//         },

//         background: {
//           default: "rgb(17, 24, 39)",
//         },
//       },
//     },
//   },
//   typography: {
//     fontFamily: [
//       "ui-sans-serif",
//       "system-ui",
//       "sans-serif",
//       "Apple Color Emoji",
//       "Segoe UI Emoji",
//       "Segoe UI Symbol",
//       "Noto Color Emoji",
//     ].join(","),
//     h1: {
//       fontSize: 30,
//       fontWeight: 600,
//     },
//     p1: {
//       fontSize: 16,
//       fontWeight: 400,
//       lineHeight: 1.5,
//     },
//   },
// });
