import axios from "axios";

const URL_BACKEND = "http://localhost:3000/";


const api = axios.create({
    baseURL: URL_BACKEND,
    withCredentials: true
});

api.interceptors.response.use(
  (response) => {

    if (response.data && response.data.data) {
      response.data = response.data.data;
    }

    return response;
  },
  (error) => {

    if (error.status == 401) {
      window.dispatchEvent(new Event('unauthorized'));
    }
    return Promise.reject(error);
  }
);

api.interceptors.request.use((config) => {
    const token = localStorage.getItem("access_token")
    if (token) {
        config.headers.Authorization = `Bearer ${token}`
    }

    return config
})

export default api;