import { reactive } from "vue";

export default class {
  constructor() {
    this.namespace = window.location.hash.substring(1);

    this.data = reactive({
      token: null,
      groupedTranslations: [],
      userId: null,
      expiredAt: null,
    });
  }

  get token() {
    return this.data.token;
  }

  get userId() {
    return this.data.userId;
  }

  get expiredAt() {
    return this.data.expiredAt;
  }

  get groupedTranslations() {
    return this.data.groupedTranslations;
  }

  applyLocalStorage() {
    // Fill store on login based on localStorage content
    if (window.localStorage.getItem("token") !== null) {
      this.data.token = localStorage.getItem("token");
      this.data.userId = localStorage.getItem("userId");
      this.data.expiredAt = localStorage.getItem("expiredAt");
    }
  }

  saveToken(token, userId, expiredAt) {
    this.data.token = token;
    this.data.userId = userId;
    this.data.expiredAt = expiredAt;

    localStorage.setItem("token", token);
    localStorage.setItem("userId", userId);
    localStorage.setItem("expiredAt", expiredAt);
  }

  resetToken() {
    this.data.token = null;
    this.data.userId = null;
    this.data.expiredAt = null;

    localStorage.removeItem("token");
    localStorage.removeItem("userId");
    localStorage.removeItem("expiredAt");
  }

  headers() {
    if (this.token) {
      return new Headers({
        Authorization: `Bearer ${this.data.token}`,
        "Content-Type": "application/json",
      });
    } else {
      return new Headers({ "Content-Type": "application/json" });
    }
  }

  callApi(url, method, data) {
    let jsonData = undefined;
    if (data !== undefined) {
      jsonData = JSON["stringify"](data);
    }

    return fetch(url, {
      headers: this.headers(),
      redirect: "follow",
      method: method ? method : "GET",
      body: jsonData,
    });
  }

  fetchTranslations() {
    return this.callApi("/api/v1/translations")
      .then((response) => {
        if (response.ok) {
          return response.json();
        } else if (response.status === 401 || response.status === 403) {
          this.resetToken();
        } else {
          console.error(
            `Received ${response.status} status while fetching translations, aborting`
          );
        }
      })
      .then((data) => {
        this.data.groupedTranslations = data;
      });
  }
}
