import { useState } from "react";
import { Button, Switch, TextField, Typography, Box } from "@mui/material";

const TestPage = () => {
  const [checked, setChecked] = useState(false);
  const [text, setText] = useState("");
  const [count, setCount] = useState(0);

  return (
    <Box sx={{ p: 4 }}>
      <Typography variant="h4" gutterBottom>
        Тестовая страница Material UI
      </Typography>
      <Box sx={{ mb: 2 }}>
        <Button variant="contained" onClick={() => setCount(count + 1)}>
          Нажми меня
        </Button>
        <Typography sx={{ mt: 1 }}>Счетчик: {count}</Typography>
      </Box>
      <Box sx={{ mb: 2 }}>
        <Switch checked={checked} onChange={() => setChecked(!checked)} />
        <Typography component="span">
          {checked ? "Включено" : "Выключено"}
        </Typography>
      </Box>
      <Box>
        <TextField
          label="Введите текст"
          value={text}
          onChange={(e) => setText(e.target.value)}
        />
        <Typography sx={{ mt: 1 }}>Вы ввели: {text}</Typography>
      </Box>
    </Box>
  );
};

export default TestPage;
