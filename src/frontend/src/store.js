import { reactive, ref } from "vue";

const data = reactive({
  token: null,
  /**
   * @type {[key as String]: Object}
   */
  groupedTranslations: {},
  userId: null,
  expiredAt: null,
});

export default class {
  get token() {
    return data.token;
  }

  get userId() {
    return data.userId;
  }

  get expiredAt() {
    return data.expiredAt;
  }

  get groupedTranslations() {
    return data.groupedTranslations;
  }

  applyLocalStorage() {
    // Fill store on login based on localStorage content
    if (window.localStorage.getItem("token") !== null) {
      data.token = localStorage.getItem("token");
      data.userId = localStorage.getItem("userId");
      data.expiredAt = localStorage.getItem("expiredAt");
    }
  }

  saveToken(token, userId, expiredAt) {
    data.token = token;
    data.userId = userId;
    data.expiredAt = expiredAt;

    localStorage.setItem("token", token);
    localStorage.setItem("userId", userId);
    localStorage.setItem("expiredAt", expiredAt);
  }

  resetToken() {
    data.token = null;
    data.userId = null;
    data.expiredAt = null;

    localStorage.removeItem("token");
    localStorage.removeItem("userId");
    localStorage.removeItem("expiredAt");
  }

  headers() {
    if (this.token) {
      return new Headers({
        Authorization: `Bearer ${this.token}`,
        "Content-Type": "application/json",
      });
    } else {
      return new Headers({ "Content-Type": "application/json" });
    }
  }

  callApi(url, method, data) {
    let jsonData = undefined;
    if (data !== undefined) {
      jsonData = JSON.stringify(data);
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
          console.error(`Received ${response.status} status while fetching translations, aborting`);
        }
      })
      .then((apiData) => {
        data.groupedTranslations = apiData;
      });
  }
}
