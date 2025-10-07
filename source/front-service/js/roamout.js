function roamOutInit() {
  console.log("💠 Roam OUT page initialized");

  // Example: clickable rows
  $("#content-area table tbody tr").on("click", function () {
    const cells = $(this).children("td").map((i, el) => $(el).text()).get();
    AppUtils.notify("Roam OUT Selected", `Country: ${cells[1]}, Operator: ${cells[2]}, Subscribers: ${cells[3]}`, "success");
  });
}
