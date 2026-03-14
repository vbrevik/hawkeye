<script lang="ts">
	import type { GraphNode, GraphEdge } from '$lib/api/types';

	interface Props {
		nodes: GraphNode[];
		edges: GraphEdge[];
		onNodeClick?: (node: GraphNode) => void;
	}

	let { nodes, edges, onNodeClick }: Props = $props();

	let canvas: HTMLCanvasElement;
	let width = $state(800);
	let height = $state(600);
	let animFrame: number;

	interface SimNode extends GraphNode {
		x: number;
		y: number;
		vx: number;
		vy: number;
		radius: number;
		pinned: boolean;
	}

	let simNodes = $state<SimNode[]>([]);
	let hoveredNode = $state<SimNode | null>(null);
	let dragNode = $state<SimNode | null>(null);
	let offsetX = $state(0);
	let offsetY = $state(0);
	let scale = $state(1);
	let panX = $state(0);
	let panY = $state(0);
	let isPanning = $state(false);
	let panStart = { x: 0, y: 0 };

	const NODE_COLORS: Record<string, string> = {
		entity: '#94a3b8',
		document: '#34d399',
		tag: '#a5b4fc',
		topic: '#f87171',
	};

	function hexAlpha(hex: string, alpha: number): string {
		return hex + Math.round(alpha * 255).toString(16).padStart(2, '0');
	}

	const NODE_RADIUS: Record<string, number> = {
		entity: 18,
		document: 22,
		tag: 14,
		topic: 14,
	};

	function initSimulation() {
		const cx = width / 2;
		const cy = height / 2;
		simNodes = nodes.map((n, i) => {
			const angle = (i / Math.max(nodes.length, 1)) * Math.PI * 2;
			const spread = Math.min(width, height) * 0.3;
			return {
				...n,
				x: cx + Math.cos(angle) * spread * (0.5 + Math.random() * 0.5),
				y: cy + Math.sin(angle) * spread * (0.5 + Math.random() * 0.5),
				vx: 0,
				vy: 0,
				radius: NODE_RADIUS[n.type] ?? 16,
				pinned: false,
			};
		});
	}

	function tick() {
		const nodeMap = new Map(simNodes.map((n) => [n.id, n]));
		const cx = width / 2;
		const cy = height / 2;

		for (const node of simNodes) {
			if (node.pinned) continue;

			// Center gravity
			node.vx += (cx - node.x) * 0.0005;
			node.vy += (cy - node.y) * 0.0005;

			// Repulsion between nodes
			for (const other of simNodes) {
				if (node === other) continue;
				const dx = node.x - other.x;
				const dy = node.y - other.y;
				const dist = Math.sqrt(dx * dx + dy * dy) || 1;
				const minDist = node.radius + other.radius + 60;
				if (dist < minDist) {
					const force = ((minDist - dist) / dist) * 0.08;
					node.vx += dx * force;
					node.vy += dy * force;
				}
			}
		}

		// Edge attraction
		for (const edge of edges) {
			const src = nodeMap.get(edge.source);
			const tgt = nodeMap.get(edge.target);
			if (!src || !tgt) continue;
			const dx = tgt.x - src.x;
			const dy = tgt.y - src.y;
			const dist = Math.sqrt(dx * dx + dy * dy) || 1;
			const targetDist = 140;
			const force = (dist - targetDist) * 0.003;
			if (!src.pinned) {
				src.vx += dx / dist * force;
				src.vy += dy / dist * force;
			}
			if (!tgt.pinned) {
				tgt.vx -= dx / dist * force;
				tgt.vy -= dy / dist * force;
			}
		}

		// Apply velocity with damping
		for (const node of simNodes) {
			if (node.pinned) continue;
			node.vx *= 0.85;
			node.vy *= 0.85;
			node.x += node.vx;
			node.y += node.vy;
			node.x = Math.max(node.radius, Math.min(width - node.radius, node.x));
			node.y = Math.max(node.radius, Math.min(height - node.radius, node.y));
		}
	}

	function draw() {
		if (!canvas) return;
		const ctx = canvas.getContext('2d');
		if (!ctx) return;

		ctx.clearRect(0, 0, width, height);
		ctx.save();
		ctx.translate(panX, panY);
		ctx.scale(scale, scale);

		const nodeMap = new Map(simNodes.map((n) => [n.id, n]));

		// Draw edges
		for (const edge of edges) {
			const src = nodeMap.get(edge.source);
			const tgt = nodeMap.get(edge.target);
			if (!src || !tgt) continue;

			ctx.beginPath();
			ctx.moveTo(src.x, src.y);
			ctx.lineTo(tgt.x, tgt.y);
			ctx.strokeStyle = 'rgba(148, 163, 184, 0.18)';
			ctx.lineWidth = 1.5;
			ctx.stroke();

			// Edge label
			const mx = (src.x + tgt.x) / 2;
			const my = (src.y + tgt.y) / 2;
			ctx.fillStyle = '#6e6e82';
			ctx.font = '9px "JetBrains Mono", monospace';
			ctx.textAlign = 'center';
			ctx.fillText(edge.label, mx, my - 4);

			// Arrow
			const angle = Math.atan2(tgt.y - src.y, tgt.x - src.x);
			const arrowDist = tgt.radius + 4;
			const ax = tgt.x - Math.cos(angle) * arrowDist;
			const ay = tgt.y - Math.sin(angle) * arrowDist;
			ctx.beginPath();
			ctx.moveTo(ax, ay);
			ctx.lineTo(ax - Math.cos(angle - 0.35) * 8, ay - Math.sin(angle - 0.35) * 8);
			ctx.lineTo(ax - Math.cos(angle + 0.35) * 8, ay - Math.sin(angle + 0.35) * 8);
			ctx.closePath();
			ctx.fillStyle = 'rgba(148, 163, 184, 0.30)';
			ctx.fill();
		}

		// Draw nodes
		for (const node of simNodes) {
			const isHovered = hoveredNode === node;
			const color = NODE_COLORS[node.type] ?? '#6366f1';

			// Glow
			if (isHovered) {
				ctx.beginPath();
				ctx.arc(node.x, node.y, node.radius + 8, 0, Math.PI * 2);
				ctx.fillStyle = hexAlpha(color, 0.15);
				ctx.fill();
			}

			// Circle
			ctx.beginPath();
			ctx.arc(node.x, node.y, node.radius, 0, Math.PI * 2);
			ctx.fillStyle = isHovered ? color : hexAlpha(color, 0.85);
			ctx.fill();
			ctx.strokeStyle = isHovered ? '#f0f0f5' : color;
			ctx.lineWidth = isHovered ? 2.5 : 1.5;
			ctx.stroke();

			// Icon text inside node
			const icon = node.type === 'document' ? '📄' : node.type === 'tag' ? '#' : node.type === 'topic' ? '◆' : '●';
			ctx.fillStyle = '#fff';
			ctx.font = `${node.radius * 0.7}px sans-serif`;
			ctx.textAlign = 'center';
			ctx.textBaseline = 'middle';
			ctx.fillText(icon, node.x, node.y);

			// Label
			ctx.fillStyle = isHovered ? '#ededf2' : '#9e9eb8';
			ctx.font = `${isHovered ? 'bold ' : ''}11px "Figtree", sans-serif`;
			ctx.textAlign = 'center';
			ctx.textBaseline = 'top';
			ctx.fillText(node.label, node.x, node.y + node.radius + 6);
		}

		ctx.restore();
	}

	function animate() {
		tick();
		draw();
		animFrame = requestAnimationFrame(animate);
	}

	function screenToWorld(sx: number, sy: number) {
		return {
			x: (sx - panX) / scale,
			y: (sy - panY) / scale,
		};
	}

	function findNode(wx: number, wy: number): SimNode | null {
		for (let i = simNodes.length - 1; i >= 0; i--) {
			const n = simNodes[i];
			const dx = wx - n.x;
			const dy = wy - n.y;
			if (dx * dx + dy * dy <= (n.radius + 4) * (n.radius + 4)) return n;
		}
		return null;
	}

	function onMouseDown(e: MouseEvent) {
		const rect = canvas.getBoundingClientRect();
		const { x, y } = screenToWorld(e.clientX - rect.left, e.clientY - rect.top);
		const node = findNode(x, y);
		if (node) {
			dragNode = node;
			node.pinned = true;
			offsetX = x - node.x;
			offsetY = y - node.y;
		} else {
			isPanning = true;
			panStart = { x: e.clientX - panX, y: e.clientY - panY };
		}
	}

	function onMouseMove(e: MouseEvent) {
		const rect = canvas.getBoundingClientRect();
		const { x, y } = screenToWorld(e.clientX - rect.left, e.clientY - rect.top);

		if (dragNode) {
			dragNode.x = x - offsetX;
			dragNode.y = y - offsetY;
			dragNode.vx = 0;
			dragNode.vy = 0;
		} else if (isPanning) {
			panX = e.clientX - panStart.x;
			panY = e.clientY - panStart.y;
		} else {
			hoveredNode = findNode(x, y);
			canvas.style.cursor = hoveredNode ? 'pointer' : 'grab';
		}
	}

	function onMouseUp() {
		if (dragNode) {
			dragNode.pinned = false;
			if (hoveredNode === dragNode && onNodeClick) {
				onNodeClick(dragNode);
			}
			dragNode = null;
		}
		isPanning = false;
	}

	function onWheel(e: WheelEvent) {
		e.preventDefault();
		const rect = canvas.getBoundingClientRect();
		const mx = e.clientX - rect.left;
		const my = e.clientY - rect.top;
		const zoom = e.deltaY > 0 ? 0.92 : 1.08;
		const newScale = Math.max(0.3, Math.min(3, scale * zoom));
		panX = mx - ((mx - panX) / scale) * newScale;
		panY = my - ((my - panY) / scale) * newScale;
		scale = newScale;
	}

	function handleResize() {
		if (!canvas) return;
		const rect = canvas.parentElement?.getBoundingClientRect();
		if (rect) {
			width = rect.width;
			height = rect.height;
		}
	}

	$effect(() => {
		if (nodes.length > 0) {
			initSimulation();
		}
	});

	$effect(() => {
		handleResize();
		animate();
		return () => cancelAnimationFrame(animFrame);
	});
</script>

<svelte:window onresize={handleResize} />

<canvas
	bind:this={canvas}
	{width}
	{height}
	onmousedown={onMouseDown}
	onmousemove={onMouseMove}
	onmouseup={onMouseUp}
	onmouseleave={onMouseUp}
	onwheel={onWheel}
	style="width: 100%; height: 100%; display: block; background: var(--bg);"
></canvas>

<style>
	canvas {
		border-radius: 0;
		border: 1px solid var(--border);
	}
</style>
