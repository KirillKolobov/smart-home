import classes from "./styles.module.scss";
import { Box, Button } from "@mui/material";
import { InputText } from "../../../../components/InputText";
import { InputTel } from "../../../../components/InputTel";

type FirstStepProps = {
  handleChangeStep: () => void;
};

export const FirstStep = ({ handleChangeStep }: FirstStepProps) => {
  return (
    <>
      <Box className={classes.nameContainer}>
        <InputText
          testId="firstName"
          name="firstName"
          label="Имя"
          rules={{
            required: "This field is required",
            maxLength: { message: "Too many symbols", value: 20 },
          }}
        />
        <InputText
          testId="lastName"
          name="lastName"
          label="Фамилия"
          rules={{
            required: "This field is required",
            maxLength: { message: "Too many symbols", value: 20 },
          }}
        />
      </Box>
      <InputText
        name="email"
        label="Email адрес"
        rules={{
          required: "This field is required",
          maxLength: { message: "Too many symbols", value: 20 },
        }}
        type="email"
        adornmentImg={"📧"}
      />
      <InputTel
        name="phone"
        label="Номер телефона (опционально)"
        placeholder="+7 (XXX) XXX-XX-XX"
        rules={{
          required: "This field is required",
          maxLength: { message: "Too many symbols", value: 11 },
        }}
        adornmentImg="📱"
      />
      <Button variant="contained" onClick={handleChangeStep}>
        Далее
      </Button>
    </>
  );
};
