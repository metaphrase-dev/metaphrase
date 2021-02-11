export default class {
  constructor() {
    this.groupedTranslations = [];
    this.namespace = window.location.hash.substring(1);

    this.token = null;
    this.userId = null;
    this.expiredAt = null;
  }

  applyLocalStorage() {
    // Fill store on login based on localStorage content
    if (window.localStorage.getItem("token") !== null) {
      this.token = localStorage.getItem("token");
      this.userId = localStorage.getItem("userId");
      this.expiredAt = localStorage.getItem("expiredAt");
    }
  }

  saveToken(token, userId, expiredAt) {
    this.token = token;
    this.userId = userId;
    this.expiredAt = expiredAt;

    localStorage.setItem("token", token);
    localStorage.setItem("userId", userId);
    localStorage.setItem("expiredAt", expiredAt);
  }

  resetToken() {
    this.token = null;
    this.userId = null;
    this.expiredAt = null;

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
        this.groupedTranslations = data;
      });
  }
}
