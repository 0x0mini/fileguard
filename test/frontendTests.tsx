import React, { useMemo } from 'react';

function expensiveCalculation(num) {
  console.log('Calculating...');
  return num * 2; // Simplistic example
}

function MyComponent({ num }) {
  const result = useMemo(() => expensiveCalculation(num), [num]); // Only re-calculated when `num` changes

  return <div>Result: {result}</div>;
}
```

```javascript
import React from 'react';

const MyExpensiveComponent = React.memo(function MyExpensiveComponent(props) {
  // Component code here
});

export default MyExpensiveComponent;
```

```javascript
import { useQuery } from 'react-query';

function useFetchData() {
  return useQuery('dataKey', () => fetch('/api/data').then(res => res.json()), {
    // Options like caching strategy go here
  });
}

function MyComponent() {
  const { data, isLoading, error } = useFetchData();

  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>An error occurred: {error.message}</div>;

  return <div>Data: {data}</usr>;
}