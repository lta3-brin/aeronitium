export const check_ocf = (ocf) => {
  switch (ocf) {
    case "raw":
      return 1
    case "EU":
      return 2
  }
}
