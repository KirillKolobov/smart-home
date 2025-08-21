import classes from "./styles.module.scss";

import { LogoBlock } from "./LogoBlock";
import { StepsBlock } from "./StepsBlok";
import { Box } from "@mui/material";
import { RegisterForm } from "./RegisterForm";
import { SecurityBlock } from "./SecurityBlock";

export const SignUpPage = () => {
  return (
    <Box className={classes.signUpPage}>
      <div className={classes.signUpContainer}>
        <LogoBlock />
        <StepsBlock />
        <RegisterForm />
        <SecurityBlock />
      </div>
    </Box>
  );
  // const [formData, setFormData] = useState({
  //   email: "",
  //   password: "",
  //   rememberMe: false,
  // });
  // const [isLoading, setIsLoading] = useState(false);
  // const [showPassword, setShowPassword] = useState(false);
  // const [error, setError] = useState("");
  // const handleSubmit = async (e) => {
  //   e.preventDefault();
  //   setIsLoading(true);
  //   setError("");
  //   // Simulate API call
  //   setTimeout(() => {
  //     if (
  //       formData.email === "admin@example.com" &&
  //       formData.password === "password"
  //     ) {
  //       alert("Успешный вход в систему!");
  //       // Redirect to dashboard
  //     } else {
  //       setError("Неверный email или пароль");
  //     }
  //     setIsLoading(false);
  //   }, 1500);
  // };
  // const handleInputChange = (e) => {
  //   const { name, value, type, checked } = e.target;
  //   setFormData((prev) => ({
  //     ...prev,
  //     [name]: type === "checkbox" ? checked : value,
  //   }));
  // };
  // return (
  //   <div className="min-h-screen bg-gray-900 flex items-center justify-center px-4">
  //     <div className="max-w-md w-full">
  //       {/* Logo Section */}
  //       <div className="text-center mb-8">
  //         <div className="w-16 h-16 bg-teal-500 rounded-full flex items-center justify-center mx-auto mb-4">
  //           <span className="text-white text-2xl font-bold">🏠</span>
  //         </div>
  //         <h1 className="text-3xl font-bold text-white mb-2">Умный дом</h1>
  //         <p className="text-gray-400">Войдите в свой аккаунт</p>
  //       </div>
  //       {/* Sign In Form */}
  //       <div className="bg-gray-800 rounded-lg p-8 shadow-xl">
  //         <form onSubmit={handleSubmit} className="space-y-6">
  //           {/* Email Field */}
  //           <div>
  //             <label
  //               htmlFor="email"
  //               className="block text-sm font-medium text-gray-300 mb-2"
  //             >
  //               Email адрес
  //             </label>
  //             <div className="relative">
  //               <input
  //                 id="email"
  //                 name="email"
  //                 type="email"
  //                 required
  //                 value={formData.email}
  //                 onChange={handleInputChange}
  //                 className="w-full px-4 py-3 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-transparent transition-colors"
  //                 placeholder="your@email.com"
  //               />
  //               <div className="absolute inset-y-0 right-0 pr-3 flex items-center">
  //                 <span className="text-gray-400">📧</span>
  //               </div>
  //             </div>
  //           </div>
  //           {/* Password Field */}
  //           <div>
  //             <label
  //               htmlFor="password"
  //               className="block text-sm font-medium text-gray-300 mb-2"
  //             >
  //               Пароль
  //             </label>
  //             <div className="relative">
  //               <input
  //                 id="password"
  //                 name="password"
  //                 type={showPassword ? "text" : "password"}
  //                 required
  //                 value={formData.password}
  //                 onChange={handleInputChange}
  //                 className="w-full px-4 py-3 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-transparent transition-colors pr-12"
  //                 placeholder="Введите пароль"
  //               />
  //               <button
  //                 type="button"
  //                 onClick={() => setShowPassword(!showPassword)}
  //                 className="absolute inset-y-0 right-0 pr-3 flex items-center text-gray-400 hover:text-white transition-colors"
  //               >
  //                 {showPassword ? "👁️" : "👁️‍🗨️"}
  //               </button>
  //             </div>
  //           </div>
  //           {/* Remember Me & Forgot Password */}
  //           <div className="flex items-center justify-between">
  //             <label className="flex items-center">
  //               <input
  //                 name="rememberMe"
  //                 type="checkbox"
  //                 checked={formData.rememberMe}
  //                 onChange={handleInputChange}
  //                 className="w-4 h-4 text-teal-500 bg-gray-700 border-gray-600 rounded focus:ring-teal-500 focus:ring-2"
  //               />
  //               <span className="ml-2 text-sm text-gray-300">
  //                 Запомнить меня
  //               </span>
  //             </label>
  //             <button
  //               type="button"
  //               className="text-sm text-teal-400 hover:text-teal-300 transition-colors"
  //             >
  //               Забыли пароль?
  //             </button>
  //           </div>
  //           {/* Error Message */}
  //           {error && (
  //             <div className="bg-red-900/50 border border-red-500 rounded-lg p-3">
  //               <p className="text-red-300 text-sm">{error}</p>
  //             </div>
  //           )}
  //           {/* Submit Button */}
  //           <button
  //             type="submit"
  //             disabled={isLoading}
  //             className="w-full bg-teal-500 hover:bg-teal-600 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-medium py-3 px-4 rounded-lg transition-colors duration-200 flex items-center justify-center"
  //           >
  //             {isLoading ? (
  //               <>
  //                 <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-white mr-2"></div>
  //                 Вход в систему...
  //               </>
  //             ) : (
  //               "Войти"
  //             )}
  //           </button>
  //         </form>
  //         {/* Divider */}
  //         <div className="mt-6">
  //           <div className="relative">
  //             <div className="absolute inset-0 flex items-center">
  //               <div className="w-full border-t border-gray-600"></div>
  //             </div>
  //             <div className="relative flex justify-center text-sm">
  //               <span className="px-2 bg-gray-800 text-gray-400">или</span>
  //             </div>
  //           </div>
  //         </div>
  //         {/* Social Login Buttons */}
  //         <div className="mt-6 space-y-3">
  //           <button
  //             type="button"
  //             className="w-full bg-gray-700 hover:bg-gray-600 text-white font-medium py-3 px-4 rounded-lg transition-colors duration-200 flex items-center justify-center"
  //           >
  //             <span className="mr-2">🔑</span>
  //             Войти через Google
  //           </button>
  //           <button
  //             type="button"
  //             className="w-full bg-gray-700 hover:bg-gray-600 text-white font-medium py-3 px-4 rounded-lg transition-colors duration-200 flex items-center justify-center"
  //           >
  //             <span className="mr-2">📱</span>
  //             Войти через Apple ID
  //           </button>
  //         </div>
  //         {/* Sign Up Link */}
  //         <div className="mt-6 text-center">
  //           <p className="text-gray-400">
  //             Нет аккаунта?{" "}
  //             <button
  //               type="button"
  //               className="text-teal-400 hover:text-teal-300 font-medium transition-colors"
  //             >
  //               Зарегистрироваться
  //             </button>
  //           </p>
  //         </div>
  //       </div>
  //       {/* Demo Credentials */}
  //       <div className="mt-6 bg-gray-800 rounded-lg p-4 border border-gray-700">
  //         <h3 className="text-white font-medium mb-2">🔍 Демо доступ:</h3>
  //         <div className="space-y-1 text-sm text-gray-300">
  //           <p>
  //             <strong>Email:</strong> admin@example.com
  //           </p>
  //           <p>
  //             <strong>Пароль:</strong> password
  //           </p>
  //         </div>
  //       </div>
  //     </div>
  //   </div>
  // );
};
