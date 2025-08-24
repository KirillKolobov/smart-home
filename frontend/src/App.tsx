import { BrowserRouter, Route, Routes } from "react-router";

import TestPage from "./pages/TestPage/TestPage";
import { theme } from "./theme";
import { SignUpPage } from "./pages/SignUp";
import { SignInPage } from "./pages/SignIn";
import {
  CssBaseline,
  StyledEngineProvider,
  ThemeProvider,
} from "@mui/material";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

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
            <Route element={<TestPage />} path="/test" />
          </Routes>
        </BrowserRouter>
        </QueryClientProvider>
      </ThemeProvider>
    </StyledEngineProvider>
  );
};

export default App;
