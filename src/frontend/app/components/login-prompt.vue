<template>
  <div class="login-dialog">
    <h2>Please login to continue.</h2>
    <form action="./" method="POST" @submit.prevent='logIn'>
      <label>
        <span>Email:</span>
        <input name="email" v-model="loginEmail" />
        <i class="fa fa-fw fa-user" />
      </label>
      <label>
        <span>Password:</span>
        <input name="password" type="password" v-model="loginPassword" />
        <i class="fa fa-fw fa-lock" />
      </label>
      <p class="error" v-if="loginError">
        <i class="fa fa-fw fa-warning"></i> {{loginError}}
      </p>
      <p class="controls">
        <button><i class="fa fa-fw fa-sign-in"></i> Login</button>
      </p>
    </form>
  </div>
</template>

<script>
export default {
  name: 'login-prompt',

  data() {
    return {
      loginEmail: '',
      loginPassword: '',
      loginError: ''
    };
  },

  methods: {
    logIn() {
      let headers = new Headers();
      headers.append("Content-Type", "application/json");

      let request = new Request('/api/v1/login', {
        method: 'POST',
        redirect: 'follow',
        headers: headers,
        body: JSON['stringify']({
          email: this.loginEmail,
          password: this.loginPassword
        })
      });

      this.loginError = '';

      fetch(request)
        .then((response) => {
          if (response.ok) {
            response.json().then((data) => {
              this.$emit("didLogIn", data.token, data.user_id, data.expired_at)

              // Clean up email/password storage after login
              this.loginEmail = '';
              this.loginPassword = '';
              this.loginError = '';
            });
          } else if (response.status == 404) {
            this.loginError = 'Unknown email address.';
          } else if (response.status == 401) {
            this.loginError = 'Incorrect password. Try again.';
          }
        });
    }
  }
};
</script>
