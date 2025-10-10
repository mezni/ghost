async function roamoutInit() {
  console.log("💠 Roam OUT page initialized");

  const data = await AppUtils.fetchData("roamout");
  const ctx = document.getElementById("roamOutChart").getContext("2d");

  new Chart(ctx, {
    type: "bar",
    data: {
      labels: data?.labels || [],
      datasets: [{
        label: "Roam OUT",
        data: data?.values || [],
        backgroundColor: "rgba(75,192,192,0.5)",
        borderColor: "green",
        borderWidth: 1
      }]
    },
    options: { responsive: true, maintainAspectRatio: false }
  });
}
