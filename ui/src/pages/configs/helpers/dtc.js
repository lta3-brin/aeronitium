import axios from "axios"

export const do_rezero = async (lrn) => {
  const URL = `${process.env.APP_ADDRESS}/rezero`

  try {
    const res = await axios.post(URL, { lrn })

    return res.data
  } catch (err) {
    return {
      code_message: "DTC commands",
      kind_message: "Error packet: indicates that command was executed with error",
      data: err.message,
    }
  }
}

export const check_scanner = async (crs, scanner_number) => {
  const URL = `${process.env.APP_ADDRESS}/scannerstatus`

  try {
    const res = await axios.post(URL, { crs, scanner_number })

    return res.data
  } catch (err) {
    return {
      code_message: "DTC commands",
      kind_message: "Error packet: indicates that command was executed with error",
      data: err.message,
    }
  }
}