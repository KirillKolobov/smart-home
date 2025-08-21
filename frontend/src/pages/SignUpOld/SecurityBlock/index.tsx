import classes from "./styles.module.scss";

export const SecurityBlock = () => (
  <div className={classes.securityInfo}>
    <h4 className={classes.title}>Безопасность данных</h4>
    <p className={classes.description}>
      Ваши данные защищены 256-битным SSL-шифрованием и хранятся в соответствии
      с GDPR.
    </p>
  </div>
);
