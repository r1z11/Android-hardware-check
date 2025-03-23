import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';
import "./App.css";

interface Info {
  Battery: string,
  CPU: string,
  Display: string,
  GPU: string,
  IMEI: string,
  Manufacturer: string,
  Model: string,
  RAM: string,
  Sensors: string,
  Storage: string
}

function Check() {
  const [output, setOutput] = useState<Info | null>(null);
  const [error, setError] = useState<string | null>(null);

  const checkHardware = async () => {
    try {
      const result: Info = await invoke('run_adb_check');

      if (result) {
        const outputObj = {
          "Battery": result?.Battery,
          "CPU": result?.CPU,
          "Display": result?.Display,
          "GPU": result?.GPU,
          "IMEI": result?.IMEI,
          "Manufacturer": result?.Manufacturer,
          "Model": result?.Model,
          "RAM": result?.RAM,
          "Sensors": result?.Sensors,
          "Storage": result?.Storage
        }
        setOutput(outputObj);

      } else {
        setOutput(null);
      }

    } catch (error) {
      setError('Error running ADB command: ' + error);
    }
  };

  return (
    <main className="container">
      <h1>Android Hardware Verification</h1>
      
      <button
        className="check-btn"
        onClick={checkHardware}>
        Run Hardware Check
      </button>

      <p>
        {!output && 'Click the button to run ADB check'}
      </p>

      {output &&
        <>
          <h4 className='label'>Manufacturer</h4>
          <p className='value'>{output?.Manufacturer}</p>
          <h4 className='label'>Model</h4>
          <p className='value'>{output?.Model}</p>
          <h4 className='label'>CPU</h4>
          <p className='value'>{output?.CPU}</p>
          <h4 className='label'>RAM</h4>
          <p className='value'>{output?.RAM}</p>
          <h4 className='label'>Storage</h4>
          <p className='value'>{output?.Storage}</p>
          <h4 className='label'>Display</h4>
          <p className='value'>{output?.Display}</p>
          <h4 className='label'>GPU</h4>
          <p className='value'>{output?.GPU}</p>
          <h4 className='label'>IMEI</h4>
          <p className='value'>{output?.IMEI}</p>
          <h4 className='label'>Sensors</h4>
          <p className='value'>{output?.Sensors}</p>
        </>
      }

      {error && <p className='error'>{error}</p>}
    </main>
  );
}

export default Check;
