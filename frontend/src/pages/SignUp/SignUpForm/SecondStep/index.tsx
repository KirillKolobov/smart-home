import classes from "./styles.module.scss";
import { Box, Button, Link } from "@mui/material";
import { InputText } from "../../../../components/InputText";
import { InputCheckbox } from "../../../../components/InputCheckbox";
import { useWatch } from "react-hook-form";
import type { IFormData } from "../../types";
import {
  DEFAULT_MAX_LENGTH_MSG,
  DEFAULT_REQUIRED_MSG,
} from "../../../../constants";

type SecondStepProps = {
  handleChangeStep: () => void;
};

export const SecondStep = ({ handleChangeStep }: SecondStepProps) => {
  const [password, repeatPassword, acceptPrivacyPolicy] = useWatch<IFormData>({
    name: ["password", "repeat_password", "accept_privacy_policy"],
  });

  return (
    <>
      <InputText
        testId="password"
        name="password"
        label="Пароль"
        type="password"
        rules={{
          required: DEFAULT_REQUIRED_MSG,
          maxLength: { message: DEFAULT_MAX_LENGTH_MSG, value: 20 },
        }}
        placeholder="Минимум 8 символов"
      />
      <InputText
        testId="repeat_password"
        type="password"
        name="repeat_password"
        label="Подтвердите пароль"
        rules={{
          required: DEFAULT_REQUIRED_MSG,
          maxLength: { message: DEFAULT_MAX_LENGTH_MSG, value: 20 },
        }}
        placeholder="Повторите пароль"
      />
      <InputCheckbox
        name="accept_privacy_policy"
        label={
          <>
            Я принимаю <Link>условия использования</Link> и{" "}
            <Link> политику конфиденциальности</Link>
          </>
        }
      />
      <Box className={classes.buttonsContainer}>
        <Button
          variant="contained"
          color="secondary"
          className={classes.backButton}
          onClick={handleChangeStep}
        >
          Назад
        </Button>
        <Button
          variant="contained"
          type="submit"
          disabled={
            acceptPrivacyPolicy === false ||
            !password ||
            !repeatPassword ||
            password !== repeatPassword
          }
        >
          Создать аккаунт
        </Button>
      </Box>
    </>
  );
};
