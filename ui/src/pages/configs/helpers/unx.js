export const check_unx = (unx) => {
  switch (unx) {
    case "psi":
      return 1
    case "Pa":
      return 3
    case "atm":
      return 6
    case "mmHg":
      return 7
    case "mmH20":
      return 8
    case "bar":
      return 9
    case "kPa":
      return 10
    case "mbar":
      return 11
  }
}
