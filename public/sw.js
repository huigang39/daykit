"use strict";

/* 改这个版本号可以让旧缓存在更新时被清掉 */
var version = "v1.0.0::";

/* 安装时预缓存的资源。先缓存应用入口 "/",其余的 JS/WASM/图标
   会在第一次联网访问时由下面的 fetch 处理器自动缓存。 */
var offlineFundamentals = ["/"];

self.addEventListener("install", function (event) {
  event.waitUntil(
    caches.open(version + "fundamentals").then(function (cache) {
      return cache.addAll(offlineFundamentals);
    })
  );
});

/* 策略:命中缓存就先返回缓存,同时后台再去网络取最新版并更新缓存
   ("先快后新")。只缓存 GET 请求。 */
self.addEventListener("fetch", function (event) {
  if (event.request.method !== "GET") return;

  event.respondWith(
    caches.match(event.request).then(function (cached) {
      var networked = fetch(event.request)
        .then(function (response) {
          var copy = response.clone();
          caches.open(version + "pages").then(function (cache) {
            cache.put(event.request, copy);
          });
          return response;
        })
        .catch(function () {
          return (
            cached ||
            new Response("<h1>离线且无缓存</h1>", {
              status: 503,
              statusText: "Service Unavailable",
              headers: new Headers({ "Content-Type": "text/html; charset=utf-8" }),
            })
          );
        });

      return cached || networked;
    })
  );
});

/* 激活时清掉非当前版本的旧缓存 */
self.addEventListener("activate", function (event) {
  event.waitUntil(
    caches.keys().then(function (keys) {
      return Promise.all(
        keys
          .filter(function (key) {
            return !key.startsWith(version);
          })
          .map(function (key) {
            return caches.delete(key);
          })
      );
    })
  );
});
