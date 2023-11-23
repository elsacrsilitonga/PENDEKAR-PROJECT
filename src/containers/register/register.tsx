import { createSignal, Component } from "solid-js";
import "./register.css";

const Register: Component = () => {
  const [nama, setNama] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [showPassword, setShowPassword] = createSignal(false);

  return (
    <div class="flex lg:flex-row flex-col min-h-[100vh] h-fit">
      <div class="res-atas flex flex-col lg:gap-16 lg:w-1/2 lg:overflow-hidden w-full p-2">
        <div class="flex sm:flex-row flex-col w-full justify-between mx-auto my-4 ">
          <a href="/">
          <div class="flex flex-row gap-4 items-center justify-center mb-4 ml-4">
            <img
              class="business-education"
              src="src/assets/img/business-education-logo-2.png"
            />
            <div class="vertical-line"></div>
            <div class="flex flex-col gap-1 justify-center">
              <div class="text-wrapper-1">PENDEKAR</div>
              <div class="text-wrapper-2">
                Pojok Education 
                <br />
                Career Scholarship
              </div>
            </div>
          </div>
          </a>
          <div class="flex flex-row lg:gap-3 xl:gap-12 gap-12 items-start justify-center text-wrapper mr-14 ml-8">
            <p><a href="/#tentangKami">Tentang Kami</a></p>
            <p><a href="/#program">Program</a></p>
            <p><a href="/#testimoni">Testimoni</a></p>
          </div>
        </div>
        <div class="rectangle p-6 m-auto">
          <p class="mulai-aksimu-untuk md:text-lg text-xl">
            <span class="span md:text-lg text-xl">Mulai Aksimu</span>
            <span class="text-wrapper-8">&nbsp;</span>
            <span class="text-wrapper-9 md:text-lg text-xl">untuk</span>
            <span class="text-wrapper-8">&nbsp;</span>
            <span class="span md:text-lg text-xl">Dunia</span>
            <span class="text-wrapper-8 md:text-lg">
              {" "}
              <br />
            </span>
            <span class="span md:text-lg text-xl">yang lebih baik</span>
            <span class="text-wrapper-8">&nbsp;</span>
            <span class="text-wrapper-9 md:text-lg text-xl">untuk Semua!</span>
          </p>
          <img class="element" src="src/assets/img/2466249-1.png" />
        </div>
        <br />
        <br />
      </div>
      <div class="res-bawah flex flex-col lg:w-1/2 w-full p-2">
        <div class="flex flex-row gap-6 items-center px-6 pt-6 pb-2 items-center">
          <img
            class="img"
            src="src/assets/img/business-education-logo-2-59B.png"
          />
          <div class="vertical-line-1"></div>
          <div class="flex flex-col justify-center">
            <div class="text-wrapper-10">PENDEKAR</div>
            <div class="text-wrapper-11 ">
              Pojok Education Career Scholarship
            </div>
          </div>
        </div>
        <div class="flex flex-col p-4 gap-3 mx-auto pr-8 w-4/5 h-full items-center justify-center">
          <p class="text-wrapper-12 self-start">Register</p>
          <div class="password-toggle w-36/37 mr-4">
            <input
              type="text"
              class="overlap-5 p-2 w-full"
              name="nama"
              placeholder="Nama Lengkap"
              value={nama()}
              onInput={(e) => setNama(e.target.value)}
            />
          </div>
          <div class="w-36/37 mr-4">
            <div class="text-black text-lg">Tanggal Lahir</div>
            <input
              type="date"
              class="overlap-5 p-2 w-full"
              placeholder="Tanggal Lahir"
            />
          </div>
          <div class="w-36/37 mr-4">
            <input
              type="text"
              class="overlap-5 p-2 w-full"
              placeholder="Nomor Telepon"
            />
          </div>
          <div class="w-36/37 mr-4">
            <input
              type="email"
              class="overlap-5 p-2 w-full"
              placeholder="Email"
            />
          </div>
          <div class="w-full mx-auto">
            <select name="" id="" class="overlap-5 p-2 w-full">
              <option value="">semester</option>
              <option value="">Semester 1</option>
              <option value="">Semester 2</option>
              <option value="">Semester 3</option>
              <option value="">Semester 4</option>
              <option value="">Semester 5</option>
              <option value="">Semester 6</option>
              <option value="">Semester 7</option>
              <option value="">Semester 8</option>
            </select>
          </div>
          <div class="w-full mx-auto">
            <select name="" id="" class="overlap-5 p-2 w-full">
              <option value="">Pendidikan Terakhir</option>
              <option value="">SD</option>
              <option value="">SMP</option>
              <option value="">SMA</option>
              <option value="">Diploma</option>
              <option value="">Sarjana</option>
              <option value="">Magister</option>
              <option value="">Doktor</option>
            </select>
          </div>
          <div class="w-36/37 mr-4">
            <input
              type="text"
              class="overlap-5 p-2 w-full"
              placeholder="Username"
            />
          </div>
          <div class="password-toggle w-36/37 mr-4">
            <input
              type={showPassword() ? "text" : "password"}
              class="overlap-5 p-2 w-full"
              name="password"
              placeholder="Password"
              value={password()}
              onInput={(e) => setPassword(e.target.value)}
            />
            <i
              class={`fas ${showPassword() ? "fa-eye" : "fa-eye-slash"}`}
              onClick={() => setShowPassword(!showPassword())}
            ></i>
          </div>
          <button class="overlap-6 px-4 py-3 mt-5 w-full">
            <div class="text-wrapper-16 login-button">Register</div>
          </button>
          <p class="belum-punya-akun">
            <span class="text-wrapper-14">Sudah punya akun? </span>{" "}
            <span class="text-wrapper-15">
              <a href="/login" id="auth">Login </a>
            </span>
          </p>
        </div>
      </div>
    </div>
  );
};

export default Register;
