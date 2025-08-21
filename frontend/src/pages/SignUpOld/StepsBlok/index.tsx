import { StepsCircles } from "./StepsCircles";
import classes from "./styles.module.scss";

export const StepsBlock = () => (
  <div className={classes.stepsContainer}>
    <StepsCircles activeStep={1} />
    <div className={classes.stepLabels}>
      <span className={classes.label}>Личные данные</span>
      <span className={classes.label}>Безопасность</span>
    </div>
  </div>
);
