self.onmessage = function(e) {
    // future heavy processing এখানে যাবে
    self.postMessage({ status: "ok" });
};
