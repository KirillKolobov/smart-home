import { BrowserRouter, Route, Routes } from "react-router";
import { theme } from "./theme";
import { SignUpPage } from "./pages/SignUp";
import {
  CssBaseline,
  StyledEngineProvider,
  ThemeProvider,
} from "@mui/material";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { SignInPage } from "./pages/SignIn";

const queryClient = new QueryClient()

const App = () => {
  return (
    <StyledEngineProvider injectFirst>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <QueryClientProvider client={queryClient}>
        <BrowserRouter>
          <Routes>
            <Route element={<SignUpPage />} path="/sign-up"></Route>
            <Route element={<SignInPage />} path="/sign-in"></Route>
          </Routes>
        </BrowserRouter>
        </QueryClientProvider>
      </ThemeProvider>
    </StyledEngineProvider>
  );
};

export default App;
