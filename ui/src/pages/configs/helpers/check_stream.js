export const precheck_stream = (refs) => {
  const scn_address = refs.scn_address
  const stbl = refs.stbl
  const msd = refs.msd
  const nms = refs.nms
  const frd = refs.frd
  const nfr = refs.nfr
  const sport = refs.sport

  if (scn_address && scn_address.hasError) { return true}
  else if (stbl && stbl.hasError) { return true }
  else if (msd && msd.hasError) { return true }
  else if (nms && nms.hasError) { return true }
  else if (frd && frd.hasError) { return true }
  else if (nfr && nfr.hasError) { return true }
  else if (sport && sport.hasError) { return true }
  else { return false }
}
