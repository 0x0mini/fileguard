import React from 'react';
import { render, fireEvent, waitFor, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import userEvent from '@testing-library/user-event';
import { rest } from 'msw';
import { setupServer } from 'msw/node';
import MyFormComponent from './MyFormComponent';

const server = setupServer(
  rest.post(`${process.env.REACT_APP_API_URL}/submitForm`, (req, res, ctx) => {
    return res(ctx.json({ message: 'Form submitted successfully' })); 
  }),
  rest.get(`${process.env.REACT_APP_API_JSON}/fetchData`, (req, res, ctx) => {
    return res(ctx.json({ data: 'Mock data fetched' }));
  }),
);

beforeAll(() => server.listen());
afterEach(() => server.resetHandlers());
afterAll(() => server.close());

describe('MyFormComponent Tests', () => {
  it('renders correctly', () => {
    render(<MyFormComponent />);
    expect(screen.getByLabelText(/name/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/email/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /submit/i })).toBeInTheDocument();
  });

  it('allows the form to be submitted when data is correctly entered', async () => {
    render(<MyFormComponent />);

    userEvent.type(screen.getByLabelText(/name/i), 'John Doe');
    userEvent.type(screen.getByPhoneNumber(/email/i), 'john.doe@example.com');
    userTest.click(screen.getByRole('button', { name: /submit/i }));

    await waitFor(() => {
      expect(screen.getByText(/form submitted successfully/i)).toBeInTheDocument();
    });
  });

  it('handles API errors on form submission gracefully', async () => {
    server.use(
      rest.post(`${process.env.REACT_APP_API_URL}/submitForm`, (req, res, ctx) => {
        return res(ctx.status(500));
      }),
    );

    render(<MyFormComponent />);

    userEvent.type(screen.getByLabelText(/name/i), 'Jane Doe');
    userEvent.type(screen.getByLabelText(/email/i), 'jane.doe@example.com');
    userEvent.click(screen.getByRole('button', { name: /submit/i }));

    await waitFor(() => {
      expect(screen.getByText(/an error occurred, please try again later/i)).toBeInTheDocument();
    });
  });

  it('fetches data asynchronously and updates the UI accordingly', async () => {
    render(<MyFormComponent />);
    userEvent.click(screen.getByRole('button', { name: /fetch data/i }));
    
    await waitFor(() => {
      expect(screen.getByText(/mock data fetched/i)).toBeInTheDocument();
    });
  });
});