<template>
  <div class="login-dialog">
    <h2>Please login to continue.</h2>
    <form action="./" method="POST" @submit.prevent="logIn">
      <label>
        <span>Email:</span>
        <input name="email" v-model="loginEmail" />
        <IconUser />
      </label>
      <label>
        <span>Password:</span>
        <input name="password" type="password" v-model="loginPassword" />
        <IconLock />
      </label>
      <p class="error" v-if="loginError"><IconWarning /> {{ loginError }}</p>
      <p class="controls">
        <button><IconLoginVariant /> Login</button>
      </p>
    </form>
  </div>
</template>

<script>
import IconUser from "/@vite-icons/mdi/user.vue";
import IconLock from "/@vite-icons/mdi/lock.vue";
import IconWarning from "/@vite-icons/mdi/warning.vue";
import IconLoginVariant from "/@vite-icons/mdi/login-variant.vue";

export default {
  name: "login-prompt",

  components: {
    IconUser,
    IconLock,
    IconWarning,
    IconLoginVariant,
  },

  data() {
    return {
      loginEmail: "",
      loginPassword: "",
      loginError: "",
    };
  },

  methods: {
    logIn() {
      this.loginError = "";

      this.$root.$props.store
        .callApi("/api/v1/login", "POST", {
          email: this.loginEmail,
          password: this.loginPassword,
        })
        .then((response) => {
          if (response.ok) {
            response.json().then((data) => {
              this.$emit("didLogIn", data.token, data.user_id, data.expired_at);

              // Clean up email/password storage after login
              this.loginEmail = "";
              this.loginPassword = "";
              this.loginError = "";
            });
          } else if (response.status == 404) {
            this.loginError = "Unknown email address.";
          } else if (response.status == 401) {
            this.loginError = "Incorrect password. Try again.";
          } else {
            response.text().then((text) => {
              this.loginError = text;
            });
          }
        });
    },
  },
};
</script>

<style>
.login-dialog {
  width: 300px;
  margin: auto;
  border: 1px solid #cccccc;
  padding: 20px;
  border-radius: 5px;
  position: relative;
  font-size: 10.5pt;
}

.login-dialog h2 {
  font-size: 12pt;
  margin-top: 0;
}

.login-dialog label {
  display: block;
  margin-top: 20px;
}

.login-dialog input {
  display: block;
  width: 100%;
  border: 1px solid #aaaaaa;
  font-family: inherit;
  font-size: 10pt;
  line-height: 1.8;
  border-radius: 4px;
  padding: 3px 5px;
  padding-left: 28px;
}

.login-dialog input + svg {
  position: absolute;
  left: 25px;
  margin-top: -1.65em;
  color: #aaaaaa;
}

.login-dialog input:active + svg,
.login-dialog input:focus + svg {
  color: #777777;
}

.login-dialog input:active,
.login-dialog input:focus {
  border-color: #777777;
}

.login-dialog label span {
  display: block;
  margin-bottom: 5px;
}

.login-dialog .controls {
  margin-top: 20px;
  margin-bottom: 0;
  text-align: right;
}

.login-dialog .controls button {
  padding: 4px 7px;
  padding-left: 4px;
  font-family: inherit;
  font-size: 10.5pt;
  border-radius: 4px;
  background-color: #eeeeee;
  border: 1px solid #aaaaaa;
  outline: none;
}

.login-dialog .controls button::-moz-focus-inner {
  border: 0;
}

.login-dialog .controls button:active,
.login-dialog .controls button:focus,
.login-dialog .controls button:hover {
  background-color: white;
}

.login-dialog .controls button:focus {
  box-shadow: 0 0 1px 1px rgba(0, 0, 0, 0.05);
  outline: none;
}

.login-dialog .controls button:active {
  box-shadow: 0 0 1px 1px rgba(0, 0, 0, 0.2);
}

.login-dialog .error {
  border: 1px solid darkred;
  color: darkred;
  border-width: 1px 0;
  background-color: #ffc8ca;
  margin: 15px -20px 5px;
  padding: 5px 20px;
}
</style>
