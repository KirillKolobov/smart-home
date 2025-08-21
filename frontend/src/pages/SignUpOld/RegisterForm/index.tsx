import { Paper } from "@mui/material";
import { FormField } from "./FormField";
import classes from "./styles.module.scss";

export const RegisterForm = () => (
  <Paper elevation={5} className={classes.signUpContainer}>
    <div className={classes.registerContainer}>
      <form className={classes.registerForm}>
        <div className={classes.fields}>
          <FormField
            label="Имя"
            id="first-name"
            placeholder="Иван"
            className={classes.firstName}
          />
          <FormField
            label="Фамилия"
            id="last-name"
            placeholder="Иванов"
            className={classes.lastName}
          />
          <FormField
            label="Email адрес"
            id="email"
            placeholder="ivan@example.com"
            className={classes.email}
          />
          <FormField
            label="Номер телефона"
            id="phone"
            placeholder="+7 (999) 123-45-67"
            className={classes.phone}
          />
        </div>
        <button type="submit" className={classes.nextButton}>
          Далее
        </button>
        <p className={classes.loginLink}>
          Уже есть аккаунт? <a href="#">Войти</a>
        </p>
      </form>
    </div>
  </Paper>
);
