import { useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { InputGroup, InputGroupAddon, InputGroupButton, InputGroupInput } from "@/components/ui/input-group";
import Editor, { DiffEditor } from "@monaco-editor/react";
import ReactECharts from "echarts-for-react";

// --- NEW IMPORTS FOR THREE.JS ---
import { Canvas, useFrame } from "@react-three/fiber";
import { OrbitControls, Grid, Environment } from "@react-three/drei";
// --------------------------------
// TanStack Table & Virtual Imports
import {
  flexRender,
  getCoreRowModel,
  useReactTable,
  getSortedRowModel,
  SortingState,
} from "@tanstack/react-table";
import { useVirtualizer } from "@tanstack/react-virtual";

import { 
  Code2, 
  Send, 
  Settings, 
  Menu, 
  FileCode, 
  Zap, 
  Package, 
  GitBranch,
  Play,
  BarChart3,
  Copy,
  GitCompare,
  Box,
  Layers,
  Search,
  ArrowUpDown
} from "lucide-react";

const DEFAULT_CODE = `function calculateSum(a, b) {
  return a + b;
}

const result = calculateSum(10, 20);
console.log(\`Result: \${result}\`);`;

const ORIGINAL_CODE = `function calculateSum(a, b) {
  return a + b;
}

const result = calculateSum(5, 15);
console.log(\`Sum: \${result}\`);`;

const chartOption = {
  title: { text: 'Performance Metrics', textStyle: { color: '#ffffff' } },
  tooltip: { trigger: 'axis', backgroundColor: 'rgba(0, 0, 0, 0.8)', borderColor: '#333', textStyle: { color: '#fff' } },
  legend: { data: ['Build Time', 'Load Time', 'Parse Time'], textStyle: { color: '#666' }, bottom: 0 },
  grid: { left: '3%', right: '4%', bottom: '15%', containLabel: true },
  xAxis: { type: 'category', data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'], axisLine: { lineStyle: { color: '#333' } }, axisLabel: { color: '#666' } },
  yAxis: { type: 'value', axisLine: { lineStyle: { color: '#333' } }, axisLabel: { color: '#666' }, splitLine: { lineStyle: { color: '#222' } } },
  series: [
    { name: 'Build Time', data: [120, 132, 101, 134, 90, 230, 210], type: 'bar', itemStyle: { color: '#3b82f6' } },
    { name: 'Load Time', data: [220, 182, 191, 234, 290, 330, 310], type: 'bar', itemStyle: { color: '#10b981' } },
    { name: 'Parse Time', data: [150, 232, 201, 154, 190, 330, 410], type: 'bar', itemStyle: { color: '#f59e0b' } }
  ]
};

// Generate 1,000 mock rows for the Virtualized Table
const MOCK_INVENTORY = Array.from({ length: 1000 }, (_, i) => ({
  id: `E-${1000 + i}`,
  name: `Structural Component ${i}`,
  status: i % 3 === 0 ? "Installed" : i % 3 === 1 ? "Pending" : "Ordered",
  load: `${Math.floor(Math.random() * 5000)}kN`,
  material: i % 2 === 0 ? "S355 Steel" : "C40/50 Concrete",
}));

// --- 3D COMPONENT ---
function BuildingModel() {
  const meshRef = useRef(null);
  
  // Rotate the building slowly
  useFrame((_state, delta) => {
    if (meshRef.current) {
      (meshRef.current as any).rotation.y += delta * 0.2;
    }
  });

  return (
    <group position={[0, 0, 0]}>
      {/* Main Building Body */}
      <mesh ref={meshRef} position={[0, 2, 0]} castShadow receiveShadow>
        <boxGeometry args={[2, 4, 2]} />
        <meshStandardMaterial 
          color="#3b82f6" 
          roughness={0.1} 
          metalness={0.8}
          transparent
          opacity={0.9} 
        />
      </mesh>
      
      {/* Building Wireframe/Edges effect */}
      <mesh ref={meshRef} position={[0, 2, 0]}>
         <boxGeometry args={[2.05, 4.05, 2.05]} />
         <meshBasicMaterial wireframe color="#60a5fa" />
      </mesh>

      {/* Base Platform */}
      <mesh position={[0, -0.1, 0]} receiveShadow>
        <cylinderGeometry args={[4, 4, 0.2, 32]} />
        <meshStandardMaterial color="#333" />
      </mesh>
    </group>
  );
}
// --------------------

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [sidebarOpen, setSidebarOpen] = useState(true);
  const [code, setCode] = useState(DEFAULT_CODE);
  const [sorting, setSorting] = useState<SortingState>([]);
  const tableContainerRef = useRef<HTMLDivElement>(null);
  
  // Added '3d' to the state type
  const [activeTab, setActiveTab] = useState<'editor' | 'compare' | 'analytics' | '3d' | 'inventory'>('editor');

  // Define table columns
  const columns = [
    { accessorKey: 'id', header: 'ID', size: 100 },
    { accessorKey: 'name', header: 'Component Name', size: 200 },
    { accessorKey: 'status', header: 'Status', size: 120 },
    { accessorKey: 'load', header: 'Load Capacity', size: 120 },
    { accessorKey: 'material', header: 'Material', size: 150 },
  ];

  // Setup React Table
  const table = useReactTable({
    data: MOCK_INVENTORY,
    columns,
    state: { sorting },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
  });

  const rows = table.getRowModel().rows;

  // Setup Row Virtualizer
  const rowVirtualizer = useVirtualizer({
    count: rows.length,
    getScrollElement: () => tableContainerRef.current,
    estimateSize: () => 40,
  });

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="min-h-screen w-screen bg-background flex flex-col">
      {/* Top Navbar */}
      <div className="border-b border-border/40 bg-background/80 backdrop-blur-md sticky top-0 z-50">
        <div className="px-4 py-3 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <button 
              onClick={() => setSidebarOpen(!sidebarOpen)}
              className="p-1.5 hover:bg-accent rounded-md transition"
            >
              <Menu className="w-5 h-5" />
            </button>
            <div className="flex items-center gap-2 border-r border-border pr-4">
              <div className="p-1.5 bg-primary/10 rounded-md">
                <Zap className="w-4 h-4 text-primary" />
              </div>
              <span className="font-semibold text-sm">EtabExtension</span>
            </div>
            <div className="flex items-center gap-1 text-xs text-muted-foreground">
              <GitBranch className="w-3 h-3" />
              <span>master</span>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <Button variant="ghost" size="sm">
              <Play className="w-4 h-4" />
              Run
            </Button>
            <Button variant="ghost" size="icon">
              <Settings className="w-4 h-4" />
            </Button>
          </div>
        </div>
      </div>

      {/* Main Layout */}
      <div className="flex flex-1 overflow-hidden">
        {/* Sidebar */}
        {sidebarOpen && (
          <div className="w-64 border-r border-border/40 bg-background/50 overflow-y-auto">
            <div className="p-4 space-y-4">
              {/* Project Info */}
              <div>
                <h3 className="text-xs font-semibold text-muted-foreground uppercase mb-3">Project</h3>
                <div className="space-y-2">
                  <div className="p-3 rounded-lg bg-card border border-border/50 hover:border-primary/50 cursor-pointer transition">
                    <div className="flex items-center gap-2 mb-1">
                      <Package className="w-4 h-4 text-primary" />
                      <span className="text-sm font-medium">Dependencies</span>
                    </div>
                    <p className="text-xs text-muted-foreground">Tauri + React + Vite</p>
                  </div>
                </div>
              </div>

              {/* Tools */}
              <div>
                <h3 className="text-xs font-semibold text-muted-foreground uppercase mb-3">Tools</h3>
                <div className="space-y-1">
                  <button className="w-full text-left px-3 py-2 rounded-md text-sm flex items-center gap-2 hover:bg-accent transition">
                    <FileCode className="w-4 h-4" />
                    Editor
                  </button>
                  <button className="w-full text-left px-3 py-2 rounded-md text-sm flex items-center gap-2 hover:bg-accent transition">
                    <Code2 className="w-4 h-4" />
                    Terminal
                  </button>
                  <button className="w-full text-left px-3 py-2 rounded-md text-sm flex items-center gap-2 hover:bg-accent transition">
                    <Settings className="w-4 h-4" />
                    Settings
                  </button>
                </div>
              </div>

              {/* Features */}
              <div>
                <h3 className="text-xs font-semibold text-muted-foreground uppercase mb-3">Features</h3>
                <div className="space-y-2 text-xs">
                  <div className="flex items-center gap-2 p-2 rounded-md bg-primary/10 border border-primary/20">
                    <div className="w-2 h-2 rounded-full bg-primary animate-pulse" />
                    <span>Hot Module Reload</span>
                  </div>
                  <div className="flex items-center gap-2 p-2 rounded-md hover:bg-accent">
                    <div className="w-2 h-2 rounded-full bg-secondary" />
                    <span>Type Safe</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Main Content */}
        <div className="flex-1 overflow-hidden flex flex-col">
          <div className="max-w-full h-full flex flex-col">
            {/* Welcome Section */}
            <div className="px-6 pt-6 pb-4">
              <h1 className="text-3xl font-bold mb-2">Welcome to EtabExtension</h1>
              <p className="text-muted-foreground">A modern engineering toolkit with Monaco Editor, Analytics & 3D Visualization</p>
            </div>

            {/* Tabs */}
            <div className="px-6 flex gap-4 border-b border-border/40 overflow-x-auto">
              <button
                onClick={() => setActiveTab('editor')}
                className={`px-4 py-3 text-sm font-medium border-b-2 transition whitespace-nowrap ${
                  activeTab === 'editor'
                    ? 'border-primary text-primary'
                    : 'border-transparent text-muted-foreground hover:text-foreground'
                }`}
              >
                <FileCode className="w-4 h-4 inline mr-2" />
                Editor
              </button>
              <button
                onClick={() => setActiveTab('compare')}
                className={`px-4 py-3 text-sm font-medium border-b-2 transition whitespace-nowrap ${
                  activeTab === 'compare'
                    ? 'border-primary text-primary'
                    : 'border-transparent text-muted-foreground hover:text-foreground'
                }`}
              >
                <GitCompare className="w-4 h-4 inline mr-2" />
                Git Compare
              </button>
              <button
                onClick={() => setActiveTab('analytics')}
                className={`px-4 py-3 text-sm font-medium border-b-2 transition whitespace-nowrap ${
                  activeTab === 'analytics'
                    ? 'border-primary text-primary'
                    : 'border-transparent text-muted-foreground hover:text-foreground'
                }`}
              >
                <BarChart3 className="w-4 h-4 inline mr-2" />
                Analytics
              </button>
              {/* --- NEW 3D TAB BUTTON --- */}
              <button
                onClick={() => setActiveTab('3d')}
                className={`px-4 py-3 text-sm font-medium border-b-2 transition whitespace-nowrap ${
                  activeTab === '3d'
                    ? 'border-primary text-primary'
                    : 'border-transparent text-muted-foreground hover:text-foreground'
                }`}
              >
                <Box className="w-4 h-4 inline mr-2" />
                3D Building
              </button>
              {/* --- INVENTORY TAB BUTTON --- */}
              <button
                onClick={() => setActiveTab('inventory')}
                className={`px-4 py-3 text-sm font-medium border-b-2 transition whitespace-nowrap ${
                  activeTab === 'inventory'
                    ? 'border-primary text-primary'
                    : 'border-transparent text-muted-foreground hover:text-foreground'
                }`}
              >
                <Layers className="w-4 h-4 inline mr-2" />
                BIM Data
              </button>
            </div>

            {/* Content Area */}
            <div className="flex-1 overflow-hidden px-6 py-4">
              {activeTab === 'editor' ? (
                <div className="space-y-4 h-full flex flex-col">
                  {/* Editor Card */}
                  <Card className="flex-1 flex flex-col border-border/50">
                    <CardHeader>
                      <div className="flex items-center justify-between">
                        <CardTitle className="text-sm flex items-center gap-2">
                          <Code2 className="w-4 h-4 text-primary" />
                          Monaco Editor
                        </CardTitle>
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => {
                            navigator.clipboard.writeText(code);
                          }}
                        >
                          <Copy className="w-4 h-4" />
                        </Button>
                      </div>
                    </CardHeader>
                    <CardContent className="flex-1 p-0 border-t border-border/50">
                      <Editor
                        height="100%"
                        defaultLanguage="javascript"
                        value={code}
                        onChange={(value) => setCode(value || '')}
                        theme="vs-dark"
                        options={{
                          minimap: { enabled: false },
                          fontSize: 14,
                          fontFamily: 'Fira Code, Courier New',
                          lineNumbers: 'on',
                          scrollBeyondLastLine: false,
                          automaticLayout: true,
                          padding: { top: 16, bottom: 16 },
                        }}
                      />
                    </CardContent>
                  </Card>

                  {/* Quick Start Card */}
                  <Card className="border-border/50">
                    <CardHeader>
                      <div className="flex items-center gap-2">
                        <Zap className="w-5 h-5 text-primary" />
                        <CardTitle className="text-sm">Execute Code</CardTitle>
                      </div>
                    </CardHeader>
                    <CardContent>
                      <form
                        className="space-y-4"
                        onSubmit={(e) => {
                          e.preventDefault();
                          greet();
                        }}
                      >
                        <div>
                          <label className="text-sm font-medium mb-2 block">Greet Function</label>
                          <InputGroup>
                            <InputGroupInput
                              id="greet-input"
                              onChange={(e) => setName(e.currentTarget.value)}
                              placeholder="Enter your name..."
                              className="text-sm"
                            />
                            <InputGroupAddon align="inline-end">
                              <InputGroupButton type="submit" size="icon-sm" title="Send">
                                <Send className="w-4 h-4" />
                              </InputGroupButton>
                            </InputGroupAddon>
                          </InputGroup>
                        </div>
                      </form>
                      {greetMsg && (
                        <div className="mt-4 p-3 rounded-md bg-primary/10 border border-primary/20">
                          <p className="text-sm font-medium text-foreground">{greetMsg}</p>
                        </div>
                      )}
                    </CardContent>
                  </Card>
                </div>
              ) : activeTab === 'compare' ? (
                <Card className="h-full border-border/50 flex flex-col">
                  <CardHeader>
                    <div className="flex items-center gap-2">
                      <GitCompare className="w-5 h-5 text-primary" />
                      <div>
                        <CardTitle className="text-sm">Git Compare - File Changes</CardTitle>
                        <CardDescription>View differences between original and modified code</CardDescription>
                      </div>
                    </div>
                  </CardHeader>
                  <CardContent className="flex-1 p-0 border-t border-border/50">
                    <DiffEditor
                      height="100%"
                      original={ORIGINAL_CODE}
                      modified={code}
                      language="javascript"
                      theme="vs-dark"
                      options={{
                        minimap: { enabled: false },
                        fontSize: 14,
                        fontFamily: 'Fira Code, Courier New',
                        lineNumbers: 'on',
                        scrollBeyondLastLine: false,
                        automaticLayout: true,
                        padding: { top: 16, bottom: 16 },
                        renderSideBySide: true,
                        wordWrap: 'on',
                      }}
                    />
                  </CardContent>
                </Card>
              ) : activeTab === 'analytics' ? (
                <div className="space-y-4 h-full">
                  <Card className="h-full border-border/50 flex flex-col">
                    <CardHeader>
                      <div className="flex items-center gap-2">
                        <BarChart3 className="w-5 h-5 text-primary" />
                        <div>
                          <CardTitle className="text-sm">Performance Analytics</CardTitle>
                          <CardDescription>Build, load, and parse time metrics</CardDescription>
                        </div>
                      </div>
                    </CardHeader>
                    <CardContent className="flex-1 p-0 border-t border-border/50">
                      <ReactECharts
                        option={chartOption}
                        style={{ width: '100%', height: '100%' }}
                        opts={{ renderer: 'canvas' }}
                      />
                    </CardContent>
                  </Card>
                </div>
              ) : activeTab === '3d' ? (
                /* --- NEW 3D TAB CONTENT --- */
                <div className="space-y-4 h-full">
                  <Card className="h-full border-border/50 flex flex-col">
                    <CardHeader>
                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-2">
                          <Box className="w-5 h-5 text-primary" />
                          <div>
                            <CardTitle className="text-sm">3D Building Renderer</CardTitle>
                            <CardDescription>Interactive architectural visualization</CardDescription>
                          </div>
                        </div>
                        <div className="flex gap-2">
                            <div className="text-xs bg-primary/20 text-primary px-2 py-1 rounded">
                                Left Click: Rotate
                            </div>
                            <div className="text-xs bg-primary/20 text-primary px-2 py-1 rounded">
                                Right Click: Pan
                            </div>
                        </div>
                      </div>
                    </CardHeader>
                    <CardContent className="flex-1 p-0 border-t border-border/50 bg-[#1e1e1e] relative">
                      {/* React Three Fiber Canvas */}
                      <Canvas
                          // Hint to the browser to use the NVIDIA/AMD GPU
                          gl={{ 
                            powerPreference: "high-performance",
                            antialias: true,
                            stencil: false,
                            depth: true
                          }}
                          dpr={[1, 2]} // Handle High DPI screens
                          shadows
                          camera={{ position: [5, 5, 5], fov: 50 }}>
                        {/* Lighting */}
                        <ambientLight intensity={0.5} />
                        <directionalLight position={[10, 10, 5]} intensity={1} castShadow />
                        
                        {/* Environment for reflections */}
                        <Environment preset="city" />

                        {/* Controls */}
                        <OrbitControls makeDefault minPolarAngle={0} maxPolarAngle={Math.PI / 1.75} />

                        {/* Scene Content */}
                        <group position={[0, -1, 0]}>
                            <Grid 
                              infiniteGrid 
                              fadeDistance={30} 
                              sectionColor="#4d4d4d" 
                              cellColor="#333" 
                            />
                            <BuildingModel />
                        </group>
                      </Canvas>
                    </CardContent>
                  </Card>
                </div>
              ) : (
                /* --- INVENTORY TAB CONTENT --- */
                <Card className="h-full flex flex-col border-border/50 overflow-hidden">
                  <CardHeader className="pb-3">
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        <Layers className="w-5 h-5 text-primary" />
                        <div>
                          <CardTitle className="text-sm">BIM Data Explorer</CardTitle>
                          <CardDescription>{MOCK_INVENTORY.length} structural components</CardDescription>
                        </div>
                      </div>
                      <div className="relative w-64">
                        <Search className="w-4 h-4 absolute left-3 top-2.5 text-muted-foreground" />
                        <input 
                          className="w-full bg-background border border-border rounded-md pl-9 pr-4 py-1.5 text-xs focus:ring-1 focus:ring-primary outline-none" 
                          placeholder="Filter components..." 
                        />
                      </div>
                    </div>
                  </CardHeader>
                  <CardContent className="flex-1 p-0 overflow-hidden border-t border-border/50">
                    <div ref={tableContainerRef} className="h-full overflow-auto">
                      <table className="w-full text-left text-sm border-collapse">
                        <thead className="sticky top-0 bg-secondary/80 backdrop-blur z-20">
                          {table.getHeaderGroups().map(hg => (
                            <tr key={hg.id} className="border-b border-border/50">
                              {hg.headers.map(header => (
                                <th 
                                  key={header.id} 
                                  onClick={header.column.getToggleSortingHandler()} 
                                  className="px-4 py-3 font-semibold cursor-pointer hover:bg-accent transition text-left"
                                  style={{ width: header.getSize() }}
                                >
                                  <div className="flex items-center gap-2">
                                    {flexRender(header.column.columnDef.header, header.getContext())}
                                    <ArrowUpDown className="w-3 h-3 opacity-50" />
                                  </div>
                                </th>
                              ))}
                            </tr>
                          ))}
                        </thead>
                        <tbody style={{ height: `${rowVirtualizer.getTotalSize()}px`, position: 'relative' }}>
                          {rowVirtualizer.getVirtualItems().map(virtualRow => {
                            const row = rows[virtualRow.index];
                            return (
                              <tr 
                                key={row.id} 
                                style={{ 
                                  position: 'absolute', 
                                  top: 0, 
                                  left: 0,
                                  width: '100%',
                                  transform: `translateY(${virtualRow.start}px)`, 
                                  height: `${virtualRow.size}px` 
                                }} 
                                className="border-b border-border/20 hover:bg-accent/30 transition-colors"
                              >
                                {row.getVisibleCells().map(cell => (
                                  <td 
                                    key={cell.id} 
                                    className="px-4 py-2 text-xs"
                                    style={{ width: cell.column.getSize() }}
                                  >
                                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                                  </td>
                                ))}
                              </tr>
                            );
                          })}
                        </tbody>
                      </table>
                    </div>
                  </CardContent>
                </Card>
              )}
            </div>
          </div>
        </div>
      </div>
    </main>
  );
}

export default App;