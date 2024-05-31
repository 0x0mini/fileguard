import React, { useState } from 'react';
import axios from 'axios';

const useFileEncryption = () => {
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [isBusy, setIsBusy] = useState<boolean>(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [progress, setProgress] = useState<number>(0); // Progress state to track upload progress

  const onFileSelectionChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files && event.target.files.length > 0) {
      setErrorMessage(null);
      setSelectedFile(event.target.files[0]);
      setProgress(0); // Reset progress on new file selection
    } else {
      // Handle case where file selection is cleared
      setSelectedFile(null);
      setErrorMessage('File selection was cleared. Please select a file.');
    }
  };

  const performFileEncryption = async () => {
    if (!selectedFile) {
      setErrorMessage('No file selected for encryption.');
      return;
    }
    setIsBusy(true);
    try {
      const formData = new FormData();
      formData.append('file', selectedFile);
      await axios.post(`${process.env.REACT_APP_BACKEND_URL}/encrypt`, formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
        onUploadProgress: (progressEvent) => {
          const percentCompleted = Math.round((progressEvent.loaded * 100) / progressEvent.total);
          setProgress(percentCompleted);
        },
      });
    } catch (err) {
      const message = err.response?.data?.message ?? 'An unexpected error occurred during file encryption.';
      setErrorMessage(message);
      setProgress(0); // Reset progress on failure
    } finally {
      setIsBusy(false);
    }
  };

  const performFileDecryption = async () => {
    if (!selectedFile) {
      setErrorMessage('No file selected for decryption.');
      return;
    }
    setIsBusy(true);
    try {
      const formData = new FormData();
      formData.append('file', selectedFile);
      await axios.post(`${process.env.REACT_APP_BACKEND_URL}/decrypt`, formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
        onUploadProgress: (progressEvent) => {
          const percentCompleted = Math.round((progressEvent.loaded * 100) / progressEvent.total);
          setProgress(percentCompleted);
        },
      });
    } catch (err) {
      const message = err.response?.data?.message ?? 'An unexpected error occurred during file decryption.';
      setErrorMessage(message);
      setProgress(0); // Reset progress on failure
    } finally {
      setIsBusy(false);
    }
  };

  return {
    onFileSelectionChange,
    performFileEncryption,
    performFileDecryption,
    isBusy,
    errorMessage,
    progress, // Expose progress for UI components to display
  };
};

export default useFileEncryption;