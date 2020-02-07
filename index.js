import React, { useState, useEffect, useRef } from "react";
import ReactDOM from "react-dom";
import {
  Slider,
  Typography,
  CssBaseline,
  Box,
  Input,
  Paper,
  Button,
  Grid,
  Switch
} from "@material-ui/core";

function App({ rust }) {
  const [n, setN] = useState(10);
  const [beta, setBeta] = useState(1.0);
  const [t_init, set_t_init] = useState(1.0);
  const [c8, setC8] = useState(true);
  const [srcImgBuffer, setSrcImgBuffer] = useState(null);
  const [srcImg, setSrcImg] = useState(null);
  const [resImg, setResImg] = useState("");
  const [markov, setMarkov] = useState(null);
  const inputRef = useRef(null);

  useEffect(() => {
    if (srcImgBuffer == null) {
      return;
    }

    const markov = rust.WasmMK.new(srcImgBuffer);
    setMarkov(markov);
  }, [srcImgBuffer]);

  useEffect(() => {
    if (markov == null) {
      return;
    }

    const timeout = setTimeout(() => {
      if (markov == null) {
        return;
      }
      const res = markov.process(n, beta, t_init, c8);
      const base64Data = btoa(String.fromCharCode.apply(null, res));
      setResImg("data:image/png;base64," + base64Data);
    }, 1000);

    return () => clearTimeout(timeout);
  }, [n, beta, t_init, c8, markov]);

  return (
    <>
      <CssBaseline />
      <Box display="flex" padding={2} height="100%">
        <Grid container spacing={2}>
          <Grid item sm={6} xs={12}>
            <Box position="absolute">
              <Input
                type="file"
                inputRef={inputRef}
                style={{ visibility: "hidden" }}
                onChange={e => {
                  const selectedFile = e.target.files[0];

                  const dataUrlReader = new FileReader();
                  dataUrlReader.addEventListener("load", e => {
                    setSrcImg(dataUrlReader.result);
                  });
                  dataUrlReader.readAsDataURL(selectedFile);

                  const arrayBufferReader = new FileReader();
                  arrayBufferReader.addEventListener("load", e => {
                    const bytes = new Uint8Array(arrayBufferReader.result);
                    setSrcImgBuffer(bytes);
                  });
                  arrayBufferReader.readAsArrayBuffer(new Blob([selectedFile]));
                }}
              />
            </Box>

            <Box
              display="flex"
              width="100%"
              height="100%"
              alignItems="center"
              justifyContent="center"
              position="relative"
            >
              {srcImg != null && (
                <img
                  src={srcImg}
                  style={{
                    objectFit: "contain",
                    width: "100%",
                    height: "auto"
                  }}
                />
              )}
              <Box
                {...(srcImg != null
                  ? { position: "absolute", top: 0, margin: "0 auto" }
                  : {})}
              >
                <Button
                  color="primary"
                  variant="contained"
                  onClick={() => inputRef.current.click()}
                >
                  Select an image
                </Button>
              </Box>
            </Box>
          </Grid>

          <Grid item sm={6} xs={12}>
            <Box
              display="flex"
              width="100%"
              height="100%"
              alignItems="center"
              justifyContent="center"
            >
              <img
                src={resImg}
                style={{ objectFit: "contain", width: "100%", height: "auto" }}
              />
            </Box>
          </Grid>

          <Grid item xs={12}>
            <Paper variant="outlined">
              <Box p={2}>
                <Grid container spacing={2}>
                  <Grid item xs={6}>
                    <Typography gutterBottom variant="caption">
                      Iterations
                    </Typography>
                    <Slider
                      value={n}
                      valueLabelDisplay="auto"
                      step={1}
                      min={1}
                      max={100}
                      onChange={(e, nv) => setN(nv)}
                    />
                    <Typography gutterBottom variant="caption">
                      β
                    </Typography>
                    <Slider
                      value={beta}
                      valueLabelDisplay="auto"
                      step={0.01}
                      min={0.01}
                      max={10.0}
                      onChange={(e, nv) => setBeta(nv)}
                    />
                  </Grid>

                  <Grid item xs={6}>
                    <Typography gutterBottom variant="caption">
                      T Init
                    </Typography>
                    <Slider
                      value={t_init}
                      valueLabelDisplay="auto"
                      step={0.01}
                      min={0.01}
                      max={10.0}
                      onChange={(e, nv) => set_t_init(nv)}
                    />
                    <Typography gutterBottom variant="caption">
                      Connexity
                    </Typography>
                    <Grid
                      component="label"
                      container
                      alignItems="center"
                      spacing={1}
                    >
                      <Grid item>4</Grid>
                      <Grid item>
                        <Switch checked={c8} onChange={(e, nv) => setC8(nv)} />
                      </Grid>
                      <Grid item>8</Grid>
                    </Grid>
                  </Grid>
                </Grid>
              </Box>
            </Paper>
          </Grid>
        </Grid>
      </Box>
    </>
  );
}

async function setup() {
  const rust = await import("./pkg/markov");
  rust.init_panic_hook();

  console.log(rust);

  ReactDOM.render(<App rust={rust} />, document.getElementById("app"));
}

setup().catch(console.error);
