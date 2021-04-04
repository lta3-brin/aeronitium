import axios from "axios"

export const do_rezero = async (addr, lrn) => {
  const URL = `http://${addr}/rezero`

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

export const check_scanner = async (addr, crs, scanner_number) => {
  const URL = `http://${addr}/scannerstatus`

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

export const setup_stream = async (addr, payload) => {
  const URL = `http://${addr}/simplesetup`

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
