async function roaminInit() {
  console.log("💠 Roam IN page initialized");

  const data = await AppUtils.fetchData("roamin");
  const ctx = document.getElementById("roamInChart").getContext("2d");

  new Chart(ctx, {
    type: "bar",
    data: {
      labels: data?.labels || [],
      datasets: [{
        label: "Roam IN",
        data: data?.values || [],
        backgroundColor: "rgba(54,162,235,0.5)",
        borderColor: "blue",
        borderWidth: 1
      }]
    },
    options: { responsive: true, maintainAspectRatio: false }
  });
}
