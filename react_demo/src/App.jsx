import React, { useState, useEffect, useRef, useCallback } from 'react';
import { 
  Play, 
  Pause, 
  RotateCcw, 
  BarChart2, 
  Map, 
  FastForward,
  ChevronRight,
  Info,
  Code,
  Sparkles,
  X,
  Settings
} from 'lucide-react';

/**
 * ==========================================
 * 0. CONSTANTS & UTILS
 * ==========================================
 */

const COLORS = {
  default: 'text-primary',
  active: 'text-warning',
  sorted: 'text-success',
  pivot: 'text-secondary',
  visited: 'text-accent',
  path: 'text-warning',
  wall: 'text-neutral-content'
};

const PSEUDOCODE = {
  bubble: [
    "do swapped = false",
    "  for i = 0 to n-1",
    "    if array[i] > array[i+1]",
    "      swap(array[i], array[i+1])",
    "      swapped = true",
    "while swapped"
  ],
  quick: [
    "for each j in low to high",
    "  if array[j] < pivot",
    "    i++",
    "    swap(array[i], array[j])",
    "swap(array[i+1], pivot)",
    "return partition_index"
  ],
  dijkstra: [
    "while unvisited is not empty",
    "  current = node with min dist",
    "  if current is target: return path",
    "  for each neighbor of current",
    "    alt = dist[current] + 1",
    "    if alt < dist[neighbor]",
    "      dist[neighbor] = alt"
  ]
};

// Helper to format explanation segments
const createSegment = (text, type = 'default') => ({ text, type });

// Helper to highlight code variables
const formatCode = (line) => {
  const parts = line.split(/(\b(?:i|j|n|low|high|pivot|current|neighbor|target|swapped|array|dist|alt|unvisited)\b)/g);
  return parts.map((part, idx) => {
    switch (part) {
      case 'i': case 'j': case 'n': case 'swapped': case 'alt': 
        return <span key={idx} className="text-warning font-bold">{part}</span>;
      case 'pivot': 
        return <span key={idx} className="text-secondary font-bold">{part}</span>;
      case 'current': case 'unvisited': 
        return <span key={idx} className="text-accent font-bold">{part}</span>;
      case 'target': 
        return <span key={idx} className="text-error font-bold">{part}</span>;
      case 'array': case 'dist': case 'neighbor': 
        return <span key={idx} className="text-info font-bold">{part}</span>;
      case 'low': case 'high':
        return <span key={idx} className="text-neutral-content font-bold">{part}</span>;
      default: 
        return part;
    }
  });
};

/**
 * ==========================================
 * 1. ALGORITHM GENERATORS (Logic + State)
 * ==========================================
 */

function* bubbleSortGenerator(arr) {
  let array = [...arr];
  let n = array.length;
  let swapped;
  
  yield { 
    array: [...array], active: [], sorted: [], 
    line: 0, 
    desc: [createSegment("Starting pass, setting swapped to false.")] 
  };

  do {
    swapped = false;
    for (let i = 0; i < n - 1; i++) {
      // Comparison Step
      yield { 
        array: [...array], active: [i, i + 1], sorted: [], 
        line: 2,
        desc: [
          createSegment("Checking if "),
          createSegment(array[i], 'active'),
          createSegment(" > "),
          createSegment(array[i+1], 'active')
        ]
      };

      if (array[i] > array[i + 1]) {
        // Swap Step
        [array[i], array[i + 1]] = [array[i + 1], array[i]];
        swapped = true;
        yield { 
          array: [...array], active: [i, i + 1], sorted: [], 
          line: 3,
          desc: [
            createSegment("Swapping "),
            createSegment(array[i+1], 'active'),
            createSegment(" and "),
            createSegment(array[i], 'active')
          ]
        };
        yield { 
          array: [...array], active: [i, i + 1], sorted: [], 
          line: 4, 
          desc: [createSegment("Marking swapped as true.")] 
        };
      }
    }
    n--;
    yield {
      array: [...array], active: [], sorted: Array.from({length: arr.length}, (_, k) => k >= n ? k : -1).filter(k=>k!==-1),
      line: 5,
      desc: [createSegment("End of pass. Checking if swap occurred.")]
    };
  } while (swapped);

  yield { 
    array: [...array], active: [], sorted: Array.from({ length: arr.length }, (_, i) => i),
    line: -1,
    desc: [createSegment("Array is fully sorted!", 'sorted')]
  };
}

function* quickSortGenerator(arr) {
  let array = [...arr];
  const sortedIndices = [];

  function* partition(low, high) {
    let pivot = array[high];
    let i = low - 1;
    
    // Highlight Pivot
    yield { 
      array: [...array], active: [high], sorted: [...sortedIndices], pivot: high,
      line: 0,
      desc: [createSegment("Chosen pivot: "), createSegment(pivot, 'pivot')]
    };

    for (let j = low; j < high; j++) {
      yield { 
        array: [...array], active: [j], sorted: [...sortedIndices], pivot: high,
        line: 1,
        desc: [createSegment("Comparing "), createSegment(array[j], 'active'), createSegment(" < pivot "), createSegment(pivot, 'pivot')]
      };

      if (array[j] < pivot) {
        i++;
        yield { 
          array: [...array], active: [i, j], sorted: [...sortedIndices], pivot: high,
          line: 2,
          desc: [createSegment("Incrementing i to "), createSegment(i, 'default')]
        };

        [array[i], array[j]] = [array[j], array[i]];
        yield { 
          array: [...array], active: [i, j], sorted: [...sortedIndices], pivot: high,
          line: 3,
          desc: [createSegment("Swapping "), createSegment(array[i], 'active'), createSegment(" and "), createSegment(array[j], 'active')]
        };
      }
    }
    [array[i + 1], array[high]] = [array[high], array[i + 1]];
    
    yield { 
      array: [...array], active: [i + 1, high], sorted: [...sortedIndices], pivot: null,
      line: 4,
      desc: [createSegment("Moving pivot to correct position index "), createSegment(i+1, 'sorted')]
    };
    
    return i + 1;
  }

  function* sort(low, high) {
    if (low <= high) {
      let pi = yield* partition(low, high);
      sortedIndices.push(pi);
      yield* sort(low, pi - 1);
      yield* sort(pi + 1, high);
    }
  }

  yield* sort(0, array.length - 1);
  yield { 
    array: [...array], active: [], sorted: Array.from({ length: arr.length }, (_, i) => i), pivot: null,
    line: 5,
    desc: [createSegment("Sort complete!", 'sorted')]
  };
}

function* dijkstraGenerator(grid, startNode, endNode) {
  const rows = grid.length;
  const cols = grid[0].length;
  const distances = Array(rows).fill().map(() => Array(cols).fill(Infinity));
  const visited = Array(rows).fill().map(() => Array(cols).fill(false));
  const previous = Array(rows).fill().map(() => Array(cols).fill(null));
  
  distances[startNode.row][startNode.col] = 0;
  const unvisitedNodes = [];

  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      unvisitedNodes.push({ row: r, col: c });
    }
  }

  while (unvisitedNodes.length > 0) {
    yield { visited: JSON.parse(JSON.stringify(visited)), path: [], line: 0, desc: [createSegment("Checking unvisited nodes...")] };

    unvisitedNodes.sort((a, b) => distances[a.row][a.col] - distances[b.row][b.col]);
    const closest = unvisitedNodes.shift();

    if (distances[closest.row][closest.col] === Infinity) break;
    if (grid[closest.row][closest.col].isWall) continue;

    visited[closest.row][closest.col] = true;
    
    yield { 
      visited: JSON.parse(JSON.stringify(visited)), path: [], 
      line: 1,
      desc: [createSegment("Visiting node "), createSegment(`[${closest.row},${closest.col}]`, 'visited'), createSegment(" with min distance.")]
    };

    if (closest.row === endNode.row && closest.col === endNode.col) {
      let current = closest;
      const path = [];
      while (current) {
        path.push(current);
        current = previous[current.row][current.col];
      }
      yield { 
        visited: JSON.parse(JSON.stringify(visited)), path, 
        line: 2,
        desc: [createSegment("Target found! Reconstructing path.", 'path')]
      };
      return;
    }

    const neighbors = [
      { r: closest.row - 1, c: closest.col },
      { r: closest.row + 1, c: closest.col },
      { r: closest.row, c: closest.col - 1 },
      { r: closest.row, c: closest.col + 1 },
    ].filter(n => n.r >= 0 && n.r < rows && n.c >= 0 && n.c < cols);

    for (const neighbor of neighbors) {
      if (!visited[neighbor.r][neighbor.c]) {
        yield { 
            visited: JSON.parse(JSON.stringify(visited)), path: [], 
            line: 3,
            desc: [createSegment("Checking neighbor "), createSegment(`[${neighbor.r},${neighbor.c}]`, 'default')]
        };

        const newDist = distances[closest.row][closest.col] + 1;
        if (newDist < distances[neighbor.r][neighbor.c]) {
          distances[neighbor.r][neighbor.c] = newDist;
          previous[neighbor.r][neighbor.c] = closest;
          yield { 
            visited: JSON.parse(JSON.stringify(visited)), path: [], 
            line: 6,
            desc: [createSegment("Updating distance for "), createSegment(`[${neighbor.r},${neighbor.c}]`, 'active'), createSegment(` to ${newDist}`)]
          };
        }
      }
    }
  }
}

/**
 * ==========================================
 * 2. UI COMPONENTS
 * ==========================================
 */

const App = () => {
  // State
  const [view, setView] = useState('sorting'); 
  const [algoType, setAlgoType] = useState('bubble');
  const [speed, setSpeed] = useState(50);
  const [isPlaying, setIsPlaying] = useState(false);
  
  // Execution State
  const [array, setArray] = useState([]);
  const [activeIndices, setActiveIndices] = useState([]);
  const [sortedIndices, setSortedIndices] = useState([]);
  const [pivotIndex, setPivotIndex] = useState(null);
  
  // Grid State
  const GRID_ROWS = 20;
  const GRID_COLS = 35;
  const [grid, setGrid] = useState([]);
  const [startNode, setStartNode] = useState({ row: 5, col: 5 });
  const [endNode, setEndNode] = useState({ row: 15, col: 30 });
  const [visitedNodes, setVisitedNodes] = useState([]);
  const [finalPath, setFinalPath] = useState([]);
  const [isDrawing, setIsDrawing] = useState(false);

  // Info State
  const [currentLine, setCurrentLine] = useState(-1);
  const [explanation, setExplanation] = useState([]);

  const generatorRef = useRef(null);
  const intervalRef = useRef(null);

  // -------------------------
  // Initialization
  // -------------------------
  const initSorting = useCallback(() => {
    const newArr = Array.from({ length: 40 }, () => Math.floor(Math.random() * 300) + 20);
    setArray(newArr);
    setActiveIndices([]);
    setSortedIndices([]);
    setPivotIndex(null);
    setCurrentLine(-1);
    setExplanation([createSegment("Ready to sort.")]);
    setIsPlaying(false);
    generatorRef.current = null;
    clearInterval(intervalRef.current);
  }, []);

  const initGrid = useCallback(() => {
    const newGrid = [];
    for (let r = 0; r < GRID_ROWS; r++) {
      const row = [];
      for (let c = 0; c < GRID_COLS; c++) {
        row.push({
          row: r, col: c, isWall: false,
          isStart: r === startNode.row && c === startNode.col,
          isEnd: r === endNode.row && c === endNode.col,
        });
      }
      newGrid.push(row);
    }
    setGrid(newGrid);
    setVisitedNodes([]);
    setFinalPath([]);
    setCurrentLine(-1);
    setExplanation([createSegment("Ready to find path.")]);
    setIsPlaying(false);
    generatorRef.current = null;
    clearInterval(intervalRef.current);
  }, [startNode, endNode]);

  useEffect(() => {
    initSorting();
    initGrid();
  }, [initSorting, initGrid]);

  // -------------------------
  // Animation Loop
  // -------------------------
  const step = useCallback(() => {
    if (!generatorRef.current) {
      if (view === 'sorting') {
        const algo = algoType === 'bubble' ? bubbleSortGenerator : quickSortGenerator;
        generatorRef.current = algo(array);
      } else {
        generatorRef.current = dijkstraGenerator(grid, startNode, endNode);
      }
    }

    const next = generatorRef.current.next();
    
    if (next.done) {
      setIsPlaying(false);
      clearInterval(intervalRef.current);
      setExplanation([createSegment("Algorithm Complete!", 'sorted')]);
      return;
    }

    const val = next.value;
    setCurrentLine(val.line);
    setExplanation(val.desc || []);

    if (view === 'sorting') {
      if (val.array) {
        setArray(val.array);
        setActiveIndices(val.active);
        setSortedIndices(val.sorted);
        setPivotIndex(val.pivot);
      }
    } else {
      if (val.visited) {
        setVisitedNodes(val.visited);
        setFinalPath(val.path);
      }
    }
  }, [view, algoType, array, grid, startNode, endNode]);

  useEffect(() => {
    if (isPlaying) {
      intervalRef.current = setInterval(step, 101 - speed);
    } else {
      clearInterval(intervalRef.current);
    }
    return () => clearInterval(intervalRef.current);
  }, [isPlaying, speed, step]);

  // -------------------------
  // Interaction Handlers
  // -------------------------
  const togglePlay = () => setIsPlaying(!isPlaying);

  const handleReset = () => {
    if (view === 'sorting') initSorting();
    else initGrid();
  };

  const handleGridInteraction = (r, c) => {
    if (isPlaying || view === 'sorting') return;
    const newGrid = [...grid];
    if (newGrid[r][c].isStart || newGrid[r][c].isEnd) return;
    newGrid[r][c].isWall = !newGrid[r][c].isWall;
    setGrid(newGrid);
  };

  const currentPseudocode = view === 'sorting' ? (algoType === 'bubble' ? PSEUDOCODE.bubble : PSEUDOCODE.quick) : PSEUDOCODE.dijkstra;

      // Effect to sync theme with parent window
      useEffect(() => {
        const darkThemes = [
          "dark", "synthwave", "halloween", "forest", "black", "luxury", "dracula", 
          "business", "night", "coffee", "dim"
        ];
  
        const applyTheme = (themeName) => {
          if (!themeName) return;
          
          document.documentElement.setAttribute('data-theme', themeName);
          
          // Also sync local dark mode state for Lucide icons and internal logic
          const isDark = darkThemes.includes(themeName) || themeName.includes('dark');
          // If the app uses a global theme state, update it here.
          // Currently App uses internal DaisyUI themes via data-theme.
        };
  
        const syncTheme = () => {
          try {
            if (window.parent && window.parent.document) {
              const parentTheme = window.parent.document.documentElement.getAttribute('data-theme');
              applyTheme(parentTheme);
            }
          } catch (e) {
            console.warn('Could not sync theme from parent:', e);
          }
        };
  
        // Initial sync
        syncTheme();
  
        // Set up message listener for cross-frame communication
        const handleMessage = (event) => {
          if (event.data && event.data.type === 'THEME_CHANGE' && event.data.theme) {
            console.log('React App received theme change:', event.data.theme);
            applyTheme(event.data.theme);
          }
        };
        window.addEventListener('message', handleMessage);
  
        return () => {
          window.removeEventListener('message', handleMessage);
        };
      }, []);
    return (
    <div className="min-h-screen bg-transparent text-base-content font-sans selection:bg-primary/30">
      
      {/* --- Header --- */}
      <header className="border-b border-base-300 bg-base-100/50 backdrop-blur-md sticky top-0 z-50">
        <div className="max-w-7xl mx-auto px-6 h-16 flex items-center justify-between">
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center shadow-lg shadow-primary/20">
              <FastForward className="text-primary-content w-5 h-5" />
            </div>
            <h1 className="text-xl font-bold tracking-tight bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">
              AlgoVis
            </h1>
          </div>
          
          <nav className="flex bg-base-200 p-1 rounded-xl border border-base-300">
            <button 
              onClick={() => { 
                setView('sorting'); 
                initSorting(); 
              }}
              className={`px-4 py-1.5 rounded-lg text-sm font-medium transition-all flex items-center gap-2 ${view === 'sorting' ? 'bg-primary text-primary-content shadow-md' : 'text-base-content/70 hover:text-base-content'}`}
            >
              <BarChart2 size={16} /> Sorting
            </button>
            <button 
              onClick={() => { 
                setView('pathfinding'); 
                initGrid();
              }}
              className={`px-4 py-1.5 rounded-lg text-sm font-medium transition-all flex items-center gap-2 ${view === 'pathfinding' ? 'bg-primary text-primary-content shadow-md' : 'text-base-content/70 hover:text-base-content'}`}
            >
              <Map size={16} /> Pathfinding
            </button>
          </nav>
        </div>
      </header>

      <main className="max-w-7xl mx-auto p-6 grid grid-cols-1 lg:grid-cols-4 gap-6">
        
        {/* --- Sidebar --- */}
        <aside className="lg:col-span-1 space-y-6 flex flex-col h-full">
          {/* Controls Panel */}
          <div className="card bg-base-200 border border-base-300 rounded-2xl p-5 shadow-xl">
            <h3 className="text-xs font-bold text-base-content/60 uppercase tracking-wider mb-4 flex items-center gap-2">
              <Settings size={14} /> Configuration
            </h3>
            
            <div className="space-y-5">
              <div className="space-y-2">
                <label className="text-sm text-base-content/70 font-medium">Algorithm</label>
                <div className="relative">
                  <select 
                    value={algoType}
                    onChange={(e) => { setAlgoType(e.target.value); handleReset(); }}
                    className="select select-bordered select-sm w-full bg-base-100 focus:ring-2 focus:ring-primary outline-none cursor-pointer"
                  >
                    {view === 'sorting' ? (
                      <>
                        <option value="bubble">Bubble Sort</option>
                        <option value="quick">Quick Sort</option>
                      </>
                    ) : (
                      <option value="dijkstra">Dijkstra's Algorithm</option>
                    )}
                  </select>
                </div>
              </div>

              <div className="space-y-3">
                <div className="flex justify-between items-center">
                  <label className="text-sm text-base-content/70 font-medium">Speed</label>
                  <span className="text-xs px-2 py-0.5 rounded bg-base-100 text-primary font-mono">{speed}%</span>
                </div>
                <input 
                  type="range" 
                  min="1" max="100" 
                  value={speed}
                  onChange={(e) => setSpeed(parseInt(e.target.value))}
                  className="range range-xs range-primary w-full"
                />
              </div>

              <div className="pt-2 flex gap-2">
                <button 
                  onClick={togglePlay}
                  className={`flex-1 btn btn-sm h-10 gap-2 ${isPlaying ? 'btn-warning btn-outline' : 'btn-primary shadow-lg shadow-primary/20'}`}
                >
                  {isPlaying ? <Pause size={18} /> : <Play size={18} />}
                  {isPlaying ? 'Pause' : 'Start'}
                </button>
                <button 
                  onClick={handleReset}
                  className="btn btn-sm h-10 btn-square btn-ghost border border-base-300 hover:bg-base-300"
                >
                  <RotateCcw size={18} />
                </button>
              </div>
            </div>
          </div>

          {/* Code & Explanation Panel */}
          <div className="card bg-base-200 border border-base-300 rounded-2xl p-0 overflow-hidden flex-1 flex flex-col min-h-[300px]">
            {/* Live Explanation */}
            <div className="p-5 border-b border-base-300 bg-base-300/30">
              <h3 className="text-xs font-bold text-base-content/60 uppercase tracking-wider mb-2 flex items-center gap-2">
                <Info size={14} /> Current Step
              </h3>
              <div className="font-mono text-sm leading-6 min-h-[3rem]">
                 {explanation.length > 0 ? (
                   explanation.map((seg, i) => (
                     <span key={i} className={`font-semibold ${COLORS[seg.type] || 'text-base-content'}`}>
                       {seg.text}
                     </span>
                   ))
                 ) : (
                   <span className="text-base-content/50 italic">Waiting to start...</span>
                 )}
              </div>
            </div>

            {/* Pseudocode Viewer */}
            <div className="flex-1 p-5 bg-base-100 font-mono text-xs overflow-y-auto">
               <h3 className="text-xs font-bold text-base-content/60 uppercase tracking-wider mb-3 flex items-center gap-2">
                 <Code size={14} /> Pseudocode
               </h3>
               <div className="space-y-1">
                 {currentPseudocode.map((line, idx) => (
                   <div 
                     key={idx} 
                     className={`px-2 py-1 rounded transition-colors duration-200 ${
                       currentLine === idx 
                         ? 'bg-primary/20 text-primary-content border-l-2 border-primary pl-1.5 font-bold' 
                         : 'text-base-content/60 border-l-2 border-transparent pl-1.5'
                     }`}
                   >
                     {formatCode(line)}
                   </div>
                 ))}
               </div>
            </div>
          </div>
        </aside>

        {/* --- Main Canvas --- */}
        <div className="lg:col-span-3 flex flex-col gap-6">
          <div className="card bg-base-200 border border-base-300 rounded-3xl p-8 shadow-2xl relative overflow-hidden flex-1 flex items-center justify-center min-h-[500px]">
            
            {/* Background Effects */}
            <div className="absolute inset-0 opacity-10 pointer-events-none">
               <div className="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] bg-primary blur-[120px] rounded-full"></div>
               <div className="absolute bottom-[-10%] right-[-10%] w-[40%] h-[40%] bg-secondary blur-[120px] rounded-full"></div>
            </div>

            {/* Visualization Layer */}
            {view === 'sorting' ? (
              <div className="flex items-end justify-center gap-1.5 w-full h-full max-h-[400px]">
                {array.map((val, idx) => {
                  const isActive = activeIndices.includes(idx);
                  const isSorted = sortedIndices.includes(idx);
                  const isPivot = pivotIndex === idx;
                  
                  let bgClass = 'bg-primary';
                  if (isPivot) bgClass = 'bg-secondary shadow-[0_0_15px_currentColor] z-10';
                  else if (isActive) bgClass = 'bg-accent scale-y-105 shadow-[0_0_15px_currentColor] z-10';
                  else if (isSorted) bgClass = 'bg-success';

                  return (
                    <div 
                      key={idx}
                      className={`w-full max-w-[15px] rounded-t-sm transition-all duration-100 ${bgClass}`}
                      style={{ height: `${(val / 320) * 100}%` }}
                    ></div>
                  );
                })}
              </div>
            ) : (
              <div 
                className="grid gap-[2px] bg-base-300 p-2 rounded-lg shadow-inner"
                style={{ gridTemplateColumns: `repeat(${GRID_COLS}, 1fr)` }}
                onMouseDown={() => setIsDrawing(true)}
                onMouseUp={() => setIsDrawing(false)}
              >
                {grid.map((row, r) => row.map((cell, c) => {
                  const isVisited = visitedNodes[r]?.[c];
                  const isPath = finalPath.some(p => p.row === r && p.col === c);
                  
                  let bgColor = 'bg-base-100';
                  if (cell.isWall) bgColor = 'bg-neutral';
                  if (cell.isStart) bgColor = 'bg-primary shadow-[0_0_10px_currentColor] z-10 scale-110';
                  if (cell.isEnd) bgColor = 'bg-secondary shadow-[0_0_10px_currentColor] z-10 scale-110';
                  if (!cell.isStart && !cell.isEnd && !cell.isWall) {
                    if (isPath) bgColor = 'bg-warning animate-pulse shadow-[0_0_5px_currentColor]';
                    else if (isVisited) bgColor = 'bg-accent/60';
                  }

                  return (
                    <div 
                      key={`${r}-${c}`}
                      onMouseEnter={() => isDrawing && handleGridInteraction(r, c)}
                      onMouseDown={() => handleGridInteraction(r, c)}
                      className={`w-4 h-4 rounded-[2px] transition-all duration-300 cursor-crosshair ${bgColor}`}
                    ></div>
                  );
                }))}
              </div>
            )}

            {/* Legend Overlay */}
            <div className="absolute bottom-6 left-8 flex gap-6 text-[10px] font-bold text-base-content/60 uppercase tracking-widest bg-base-200/80 p-3 rounded-xl border border-base-300 backdrop-blur-sm">
               {view === 'sorting' ? (
                 <>
                   <div className="flex items-center gap-2"><div className="w-2 h-2 rounded-full bg-primary"></div> Default</div>
                   <div className="flex items-center gap-2"><div className="w-2 h-2 rounded-full bg-accent"></div> Compare</div>
                   <div className="flex items-center gap-2"><div className="w-2 h-2 rounded-full bg-success"></div> Sorted</div>
                   {algoType === 'quick' && <div className="flex items-center gap-2"><div className="w-2 h-2 rounded-full bg-secondary"></div> Pivot</div>}
                 </>
               ) : (
                 <>
                   <div className="flex items-center gap-2"><div className="w-2 h-2 rounded-full bg-accent/60"></div> Visited</div>
                   <div className="flex items-center gap-2"><div className="w-2 h-2 rounded-full bg-warning"></div> Path</div>
                   <div className="flex items-center gap-2"><div className="w-2 h-2 rounded-full bg-neutral"></div> Wall</div>
                 </>
               )}
            </div>
          </div>
        </div>
      </main>
    </div>
  );
};

export default App;
