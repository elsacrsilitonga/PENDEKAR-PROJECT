import { createSignal, Component, onMount } from 'solid-js';
import { Routes, Route, Router, useNavigate } from '@solidjs/router';
import Login from './containers/login/login';
import Register from './containers/register/register';
import { useStore } from './store';

interface UserData {
  email: string;
  password: string;
}

const App: Component = () => {
  const [{ sessionStore }] = useStore();
  const navigate = useNavigate();
  const [needLogin, setNeedLogin] = createSignal(true);

  onMount(() => {
    console.log('session ', sessionStore.sessionData);
    if (sessionStore.sessionData) {
      setNeedLogin(!needLogin());
    }
    console.log('need login ' + needLogin());
    if (needLogin()) {
      navigate('/login', { replace: true });
    }
  });

  onMount(() => {
    console.log('session ', sessionStore.sessionData);
    if (sessionStore.sessionData) {
      setNeedLogin(!needLogin());
    }
    console.log('need login ' + needLogin());
    if (needLogin()) {
      navigate('/dashboard/login', { replace: true });
    }
  });

  function getPagebyAccess() {
    const userDataString = sessionStore.sessionData as unknown as string; // Ensure sessionData is a string
    const userData = JSON.parse(userDataString) as UserData; // Parse the JSON string to an object
  }

  return (
    <>
      <Router>
        <Routes>
          <Route path="/login" element={<Login />} />
          <Route path="/register" element={<Register />} />
          {/* <Route
            path="/"
            element={!needLogin() ? getPageByAccess() : <Login />}
          /> */}
        </Routes>
      </Router>
    </>
  );
};

export default App;



//   console.log('session ', sessionStore.sessionData);
//   if (sessionStore.sessionData) {
//     setNeedLogin(false);
//     console.log('need login ' + needLogin());

//     const userDataString = sessionStore.sessionData as unknown as string; // Ensure sessionData is a string
//     const userData = JSON.parse(userDataString) as UserData; // Parse the JSON string to an object
//     const userAccess = userData.access;

//     // Redirect the user based on their access role
//     switch (userAccess) {
//       case 'admin':
//         navigate('/dashboard/report', { replace: true });
//         break;
//       case 'direktur_keuangan':
//         navigate('/dashboard/direktur_keuangan', { replace: true });
//         break;
//       case 'direktur_utama':
//         navigate('/dashboard-du/report', { replace: true });
//         break;
//       // Add more cases for other roles as needed
//       default:
//         // Handle unknown or unsupported roles here
//         break;
//     }

//   }
// })