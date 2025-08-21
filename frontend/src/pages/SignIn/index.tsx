import { useState } from "react";
// import { Eye, EyeOff, Home } from "lucide-react";
import classes from "./styles.module.scss";

export const SignInPage = () => {
  const [email, setEmail] = useState("your@email.com");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [rememberMe, setRememberMe] = useState(false);

  return (
    <div className={classes.root}>
      <div className={classes.container}>
        {/* Header */}
        <div className={classes.header}>
          <div className={classes.logo}>
            {/* <Home size={32} color="#fff" /> */}
          </div>
          <h1 className={classes.title}>Умный дом</h1>
          <p className={classes.subtitle}>Войдите в свой аккаунт</p>
        </div>

        {/* Form */}
        <div className={classes.formWrapper}>
          <div>
            {/* Email Field */}
            <div className={classes.formGroup}>
              <label className={classes.label}>Email адрес</label>
              <input
                type="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                className={classes.input}
                placeholder="your@email.com"
              />
            </div>

            {/* Password Field */}
            <div className={classes.formGroup}>
              <label className={classes.label}>Пароль</label>
              <div className={classes.passwordWrapper}>
                <input
                  type={showPassword ? "text" : "password"}
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  className={classes.input}
                  placeholder="Введите пароль"
                />
                <button
                  type="button"
                  onClick={() => setShowPassword(!showPassword)}
                  className={classes.passwordToggle}
                  tabIndex={-1}
                >
                  {/* {showPassword ? <EyeOff size={20} /> : <Eye size={20} />} */}
                </button>
              </div>
            </div>

            {/* Remember Me & Forgot Password */}
            <div className={classes.options}>
              <label className={classes.checkboxLabel}>
                <input
                  type="checkbox"
                  checked={rememberMe}
                  onChange={(e) => setRememberMe(e.target.checked)}
                  className={classes.checkbox}
                />
                <span>Запомнить меня</span>
              </label>
              <a href="#" className={classes.forgotLink}>
                Забыли пароль?
              </a>
            </div>

            {/* Sign In Button */}
            <button type="submit" className={classes.button}>
              Войти
            </button>

            {/* Divider */}
            <div className={classes.dividerWrapper}>
              <div className={classes.divider}></div>
              <div className={classes.dividerText}>или</div>
            </div>

            {/* Social Login Buttons */}
            <div className={classes.socialButtons}>
              <button type="button" className={classes.socialButton}>
                <span style={{ marginRight: 8 }}>🔑</span>
                Войти через Google
              </button>
              <button type="button" className={classes.socialButton}>
                <span style={{ marginRight: 8 }}>🍎</span>
                Войти через Apple ID
              </button>
            </div>

            {/* Sign Up Link */}
            <div className={classes.signUpLinkWrapper}>
              <span className={classes.subtitle}>Нет аккаунта? </span>
              <a href="#" className={classes.signUpLink}>
                Зарегистрироваться
              </a>
            </div>
          </div>
        </div>

        {/* Demo Access */}
        <div className={classes.demoAccess}>
          <div className={classes.demoHeader}>
            <span style={{ marginRight: 8 }}>🔓</span>
            <span>Демо доступ:</span>
          </div>
          <div className={classes.demoInfo}>
            <div>
              <strong>Email:</strong> admin@example.com
            </div>
            <div>
              <strong>Пароль:</strong> password
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
