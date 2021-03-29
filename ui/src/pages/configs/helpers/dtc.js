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

export const setup_stream = async (payload) => {
  const URL = `${process.env.APP_ADDRESS}/simplesetup`

  try {
    const res = await axios.post(URL, {
      crs: payload.crs,
      num_channels: payload.num_channels,
      scn_address: payload.scn_address,
      lrn: payload.lrn,
      stbl: payload.stbl,
      sport: payload.sport,
      nfr: payload.nfr,
      frd: payload.frd,
      nms: payload.nms,
      msd: payload.msd,
      trm: payload.trm,
      scm: payload.scm,
      ocf: payload.ocf,
      unx: payload.unx
    })

    return res.data
  } catch (err) {
    return {
      code_message: "DTC commands",
      kind_message: "Error packet: indicates that command was executed with error",
      data: err.message,
    }
  }
}
