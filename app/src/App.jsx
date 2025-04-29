import React from 'react';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';

function Home() {
  return <h2>🏠 Home Page</h2>;
}

function About() {
  return <h2>ℹ️ About Page</h2>;
}

function Contact() {
  return <h2>📞 Contact Page</h2>;
}

function App() {
  return (
    <Router>
      <div>
        <nav style={{ padding: '1rem', background: '#eee' }}>
          <Link to="/" style={{ marginRight: '1rem' }}>Home</Link>
          <Link to="/about" style={{ marginRight: '1rem' }}>About</Link>
          <Link to="/contact">Contact</Link>
        </nav>

        <div style={{ padding: '2rem' }}>
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/about" element={<About />} />
            <Route path="/contact" element={<Contact />} />
          </Routes>
        </div>
      </div>
    </Router>
  );
}

export default App;
